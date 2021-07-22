use mathml_rs::evaluate_node;

use crate::{
    Compartment, FunctionDefinition, MathNode, MathTag, Parameter, Reaction, Species,
    SpeciesReference, SpeciesStatus, Tag, UnitDefinition,
};
use std::collections::HashMap;
use std::fmt;

// An SBML Model container
#[derive(Debug, Default)]
pub struct Model {
    pub name: Option<String>,
    pub nodes: Vec<Tag>,
}

macro_rules! objects_from_list {
    ($parent_type: ident, $parent_field: ident, $node_type: ident, $node_field: ident) => {
        pub fn $node_field(&self) -> Vec<$node_type> {
            let mut result = Vec::new();
            let mut list_idx = None;
            if let Tag::Root(root) = &self.nodes[0] {
                list_idx = root.$parent_field;
            }
            if let Some(idx) = list_idx {
                if let Tag::$parent_type(list_tag) = &self.nodes[idx] {
                    for node_idx in &list_tag.$node_field {
                        if let Tag::$node_type(node) = &self.nodes[node_idx.to_owned()] {
                            result.push(node.clone());
                        }
                    }
                }
            }
            result
        }
    };
}

impl Model {
    pub fn new(nodes: Vec<Tag>) -> Self {
        Model { name: None, nodes }
    }

    objects_from_list!(ListOfSpecies, list_of_species, Species, species);
    objects_from_list!(ListOfReactions, list_of_reactions, Reaction, reactions);
    objects_from_list!(
        ListOfUnitDefinitions,
        list_of_unit_definitions,
        UnitDefinition,
        unit_definitions
    );
    objects_from_list!(
        ListOfCompartments,
        list_of_compartments,
        Compartment,
        compartments
    );
    objects_from_list!(ListOfParameters, list_of_parameters, Parameter, parameters);
    objects_from_list!(
        ListOfFunctionDefinitions,
        list_of_function_definitions,
        FunctionDefinition,
        function_definitions
    );

    pub fn function_definition_tags(&self) -> HashMap<String, Vec<MathNode>> {
        let mut tags = HashMap::new();
        for function_definition in self.function_definitions() {
            let id = function_definition.id.as_ref().unwrap().to_owned();
            let math_tag = function_definition.math_tag(&self).unwrap();
            tags.insert(id, math_tag.nodes);
        }
        tags
    }

    // Creates a HashMap from Parameters, Species and Compartments
    pub fn assignments(&self) -> HashMap<String, f64> {
        let mut hm = HashMap::<String, f64>::new();

        // Compartments
        let mut lo_comp_idx = None;
        if let Tag::Root(root) = &self.nodes[0] {
            lo_comp_idx = root.list_of_compartments;
        }
        if let Some(idx) = lo_comp_idx {
            if let Tag::ListOfCompartments(list_of_compartments) = &self.nodes[idx] {
                for comp_idx in &list_of_compartments.compartments {
                    if let Tag::Compartment(comp) = &self.nodes[comp_idx.to_owned()] {
                        if let Some(id) = comp.id.to_owned() {
                            if let Some(size) = comp.size {
                                hm.insert(id, size);
                            }
                        }
                    }
                }
            }
        }

        // Species
        let mut lo_sp_idx = None;
        if let Tag::Root(root) = &self.nodes[0] {
            lo_sp_idx = root.list_of_species;
        }
        if let Some(idx) = lo_sp_idx {
            if let Tag::ListOfSpecies(list_of_species) = &self.nodes[idx] {
                for sp_idx in &list_of_species.species {
                    if let Tag::Species(sp) = &self.nodes[sp_idx.to_owned()] {
                        if let Some(id) = sp.id.to_owned() {
                            // A species can only have one of initial_amount and
                            // initial_concentration
                            // According to the spec, "setting both is an error"
                            if let Some(initial_amount) = sp.initial_amount {
                                if sp.initial_concentration.is_none() {
                                    // If amount is set, the get concentration
                                    let compartment = sp.compartment.as_ref().unwrap();
                                    let compartment_size = hm.get(compartment).unwrap();
                                    let concentration = initial_amount / compartment_size;
                                    hm.insert(id, concentration);
                                }
                            } else if let Some(initial_concentration) = sp.initial_concentration {
                                hm.insert(id, initial_concentration);
                            }
                        }
                    }
                }
            }
        }

        // Parameters
        let mut lo_param_idx = None;
        if let Tag::Root(root) = &self.nodes[0] {
            lo_param_idx = root.list_of_parameters;
        }
        if let Some(idx) = lo_param_idx {
            if let Tag::ListOfParameters(list_of_parameters) = &self.nodes[idx] {
                for param_idx in &list_of_parameters.parameters {
                    if let Tag::Parameter(param) = &self.nodes[param_idx.to_owned()] {
                        if let Some(id) = param.id.to_owned() {
                            if let Some(value) = param.value {
                                hm.insert(id, value);
                            }
                        }
                    }
                }
            }
        }

        let function_definitions = self.function_definition_tags();

        // Initial Assignments
        let mut lo_init_assignment_idx = None;
        if let Tag::Root(root) = &self.nodes[0] {
            lo_init_assignment_idx = root.list_of_initial_assignments;
        }
        if let Some(idx) = lo_init_assignment_idx {
            if let Tag::ListOfInitialAssignments(list_of_initial_assignments) = &self.nodes[idx] {
                for init_assignment_idx in &list_of_initial_assignments.initial_assignments {
                    if let Tag::InitialAssignment(init_assignment) =
                        &self.nodes[init_assignment_idx.to_owned()]
                    {
                        if let Some(symbol) = init_assignment.symbol.to_owned() {
                            if let Some(math_tag) = init_assignment.math_tag(self) {
                                let value =
                                    evaluate_node(&math_tag.nodes, 0, &hm, &function_definitions)
                                        .expect("Evaluation failed for initial assignment.");
                                hm.insert(symbol, value);
                            }
                        }
                    }
                }
            }
        }

        hm
    }

    pub fn all_reactants(&self) -> HashMap<String, Vec<SpeciesReference>> {
        let mut result = HashMap::new();
        let reactions = self.reactions();
        for reaction in reactions {
            let rxn_id = reaction.id.as_ref().unwrap().to_owned();
            result.insert(rxn_id, reaction.reactants(&self));
        }

        result
    }

    pub fn all_reactant_ids(&self) -> HashMap<String, Vec<String>> {
        let mut result = HashMap::new();
        let reactions = self.reactions();
        for reaction in reactions {
            let rxn_id = reaction.id.as_ref().unwrap().to_owned();
            result.insert(rxn_id, reaction.reactant_ids(&self));
        }

        result
    }

    pub fn all_products(&self) -> HashMap<String, Vec<SpeciesReference>> {
        let mut result = HashMap::new();
        let reactions = self.reactions();
        for reaction in reactions {
            let rxn_id = reaction.id.as_ref().unwrap().to_owned();
            result.insert(rxn_id, reaction.products(&self));
        }

        result
    }

    pub fn all_product_ids(&self) -> HashMap<String, Vec<String>> {
        let mut result = HashMap::new();
        let reactions = self.reactions();
        for reaction in reactions {
            let rxn_id = reaction.id.as_ref().unwrap().to_owned();
            result.insert(rxn_id, reaction.product_ids(&self));
        }

        result
    }

    pub fn all_kinetic_laws(&self) -> HashMap<String, MathTag> {
        let mut result = HashMap::new();
        let reactions = self.reactions();
        for reaction in reactions {
            let rxn_id = reaction.id.as_ref().unwrap().to_owned();
            result.insert(rxn_id, reaction.kinetic_law(&self).unwrap());
        }

        result
    }

    pub fn local_parameters(&self) -> HashMap<String, HashMap<String, f64>> {
        let mut hm: HashMap<String, HashMap<String, f64>> = HashMap::new();
        for reaction in self.reactions() {
            let rxn_id = reaction.id.as_ref().unwrap().to_owned();
            let local_parameters = reaction.local_parameters(self);
            hm.insert(rxn_id, local_parameters);
        }
        hm
    }

    pub fn reaction_matrix(&self) -> HashMap<(String, String), SpeciesStatus> {
        let mut rxn_matrix: HashMap<(String, String), SpeciesStatus> = HashMap::new();

        let species = &self.species();
        let reactions = &self.reactions();
        let all_reactants = &self.all_reactants();
        let all_products = &self.all_products();

        for sp in species {
            let sp_id = sp.id.as_ref().unwrap().to_owned();
            for reaction in reactions {
                let rxn_id = reaction.id.as_ref().unwrap().to_owned();
                let reactants = all_reactants.get(&rxn_id).unwrap();
                let products = all_products.get(&rxn_id).unwrap();
                let mut inserted = false;

                for reactant in reactants {
                    if sp.id == reactant.species {
                        let stoichiometry = reactant.stoichiometry.unwrap();
                        rxn_matrix.insert(
                            (sp_id.clone(), rxn_id.clone()),
                            SpeciesStatus::Reactant(stoichiometry),
                        );
                        inserted = true;
                    }
                }

                for product in products {
                    if sp.id == product.species {
                        let stoichiometry = product.stoichiometry.unwrap();
                        rxn_matrix.insert(
                            (sp_id.clone(), rxn_id.clone()),
                            SpeciesStatus::Product(stoichiometry),
                        );
                        inserted = true;
                    }
                }

                if !inserted {
                    rxn_matrix.insert((sp_id.clone(), rxn_id.clone()), SpeciesStatus::None);
                }
            }
        }

        rxn_matrix
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Model")
    }
}
