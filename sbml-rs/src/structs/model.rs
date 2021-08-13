use std::collections::HashMap;

use crate::{
    AssignmentRule, Compartment, FunctionDefinition, InitialAssignment, MathNode, MathTag,
    Parameter, RateRule, Reaction, Species, SpeciesReference, SpeciesStatus, Tag, UnitDefinition,
};
use std::fmt;

// An SBML Model container
#[derive(Clone, Debug, Default)]
pub struct Model {
    pub id: Option<String>,
    pub name: Option<String>,
    pub meta_id: Option<String>,
    pub substance_units: Option<String>,
    pub time_units: Option<String>,
    pub volume_units: Option<String>,
    pub area_units: Option<String>,
    pub length_units: Option<String>,
    pub extent_units: Option<String>,
    pub conversion_factor: Option<String>,
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
    pub fn new(nodes: Vec<Tag>, attributes: HashMap<String, String>) -> Self {
        let mut model = Model {
            id: None,
            name: None,
            meta_id: None,
            substance_units: None,
            volume_units: None,
            time_units: None,
            extent_units: None,
            area_units: None,
            length_units: None,
            conversion_factor: None,
            nodes,
        };
        for (key, value) in attributes {
            match key.as_str() {
                "id" => model.id = Some(value),
                "name" => model.name = Some(value),
                "metaid" => model.meta_id = Some(value),
                "substanceUnits" => model.substance_units = Some(value),
                "timeUnits" => model.time_units = Some(value),
                "areaUnits" => model.area_units = Some(value),
                "lengthUnits" => model.length_units = Some(value),
                "extentUnits" => model.extent_units = Some(value),
                "volumeUnits" => model.volume_units = Some(value),
                "conversion_factor" => model.conversion_factor = Some(value),
                _ => panic!("Invalid attribute {} for model.", key),
            }
        }
        model
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
    objects_from_list!(ListOfRules, list_of_rules, AssignmentRule, assignment_rules);
    objects_from_list!(ListOfRules, list_of_rules, RateRule, rate_rules);
    objects_from_list!(
        ListOfInitialAssignments,
        list_of_initial_assignments,
        InitialAssignment,
        initial_assignments
    );

    pub fn function_definition_math(&self) -> HashMap<String, Vec<MathNode>> {
        let mut tags = HashMap::new();
        for function_definition in self.function_definitions() {
            let id = function_definition.id.as_ref().unwrap().to_owned();
            let math_tag = function_definition.math_tag(&self).unwrap();
            tags.insert(id, math_tag.nodes);
        }
        tags
    }

    pub fn assignment_rule_math(&self) -> HashMap<String, Vec<MathNode>> {
        let mut tags = HashMap::new();
        for assignment_rule in self.assignment_rules() {
            let variable = assignment_rule.variable.as_ref().unwrap().to_owned();
            let math_tag = assignment_rule.math_tag(&self).unwrap();
            tags.insert(variable, math_tag.nodes);
        }
        tags
    }

    //Creates a HashMap from Parameters, Species and Compartments
    //pub fn bindings(&self) -> HashMap<String, BindingValue> {
    //let mut bindings = HashMap::<String, BindingValue>::new();

    //Compartments
    //let mut lo_comp_idx = None;
    //if let Tag::Root(root) = &self.nodes[0] {
    //lo_comp_idx = root.list_of_compartments;
    //}
    //if let Some(idx) = lo_comp_idx {
    //if let Tag::ListOfCompartments(list_of_compartments) = &self.nodes[idx] {
    //for comp_idx in &list_of_compartments.compartments {
    //if let Tag::Compartment(comp) = &self.nodes[comp_idx.to_owned()] {
    //if let Some(id) = comp.id.to_owned() {
    //let mut binding_value = BindingValue::new(BindingType::Compartment);
    //if let Some(size) = comp.size {
    //binding_value.set(size);
    //}
    //hm.insert(id, binding_value);
    //}
    //}
    //}
    //}
    //}

    //Species
    //let mut lo_sp_idx = None;
    //if let Tag::Root(root) = &self.nodes[0] {
    //lo_sp_idx = root.list_of_species;
    //}
    //if let Some(idx) = lo_sp_idx {
    //if let Tag::ListOfSpecies(list_of_species) = &self.nodes[idx] {
    //for sp_idx in &list_of_species.species {
    //if let Tag::Species(sp) = &self.nodes[sp_idx.to_owned()] {
    //if let Some(id) = sp.id.to_owned() {
    //let mut binding_value = BindingValue::new(BindingType::SpeciesConc);
    //A species can only have one of initial_amount and
    //initial_concentration, "setting both is an error"
    //Store whatever is set here, don't convert to conc
    //Conversion is left to the simulator because compartment
    //size can change
    //if let Some(initial_amount) = sp.initial_amount {
    //if sp.initial_concentration.is_none() {
    //binding_value = BindingValue::new(BindingType::SpeciesAmt);
    //binding_value.set(initial_amount);
    //}
    //} else if let Some(initial_concentration) = sp.initial_concentration {
    //binding_value = BindingValue::new(BindingType::SpeciesConc);
    //binding_value.set(initial_concentration);
    //}
    //if none of these is set, check the hasOnlySubstanceUnits attr
    //else if let Some(true) = sp.has_only_substance_units {
    //binding_value = BindingValue::new(BindingType::SpeciesAmt);
    //}

    //set constant attribute if appropriate
    //if let Some(true) = sp.constant {
    //binding_value.constant = true;
    //}

    //store value
    //hm.insert(id, binding_value);
    //}
    //}
    //}
    //}
    //}

    //Parameters
    //let mut lo_param_idx = None;
    //if let Tag::Root(root) = &self.nodes[0] {
    //lo_param_idx = root.list_of_parameters;
    //}
    //if let Some(idx) = lo_param_idx {
    //if let Tag::ListOfParameters(list_of_parameters) = &self.nodes[idx] {
    //for param_idx in &list_of_parameters.parameters {
    //if let Tag::Parameter(param) = &self.nodes[param_idx.to_owned()] {
    //if let Some(id) = param.id.to_owned() {
    //let mut binding_value = BindingValue::new(BindingType::Parameter);
    //if let Some(value) = param.value {
    //binding_value.set(value);
    //}
    //hm.insert(id, binding_value);
    //}
    //}
    //}
    //}
    //}

    //let function_definitions = self.function_definition_math();

    //Get values from hm
    //let mut hm_values = HashMap::new();
    //for (id, binding_value) in &hm {
    //if let Some(value) = binding_value.value {
    //hm_values.insert(id.clone(), value);
    //}
    //}

    //Initial Assignments
    //let mut lo_init_assignment_idx = None;
    //if let Tag::Root(root) = &self.nodes[0] {
    //lo_init_assignment_idx = root.list_of_initial_assignments;
    //}
    //if let Some(idx) = lo_init_assignment_idx {
    //if let Tag::ListOfInitialAssignments(list_of_initial_assignments) = &self.nodes[idx] {
    //for init_assignment_idx in &list_of_initial_assignments.initial_assignments {
    //if let Tag::InitialAssignment(init_assignment) =
    //&self.nodes[init_assignment_idx.to_owned()]
    //{
    //if let Some(symbol) = init_assignment.symbol.to_owned() {
    //if let Some(math_tag) = init_assignment.math_tag(self) {
    //let value = evaluate_node(
    //&math_tag.nodes,
    //0,
    //&hm_values,
    //&function_definitions,
    //)
    //.expect("Evaluation failed for initial assignment.");
    //update values in HM
    //hm_values.entry(symbol.clone()).and_modify(|v| *v = value);
    //hm.entry(symbol).and_modify(|v| v.value = Some(value));
    //}
    //}
    //}
    //}
    //}
    //}

    //hm
    //}

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

    pub fn local_parameter_values(&self) -> HashMap<String, HashMap<String, f64>> {
        let mut hm: HashMap<String, HashMap<String, f64>> = HashMap::new();
        for reaction in self.reactions() {
            let rxn_id = reaction.id.as_ref().unwrap().to_owned();
            let local_parameters = reaction.local_parameter_values(self);
            hm.insert(rxn_id, local_parameters);
        }
        hm
    }

    pub fn reaction_matrix(&self) -> HashMap<(String, String), Vec<SpeciesStatus>> {
        let mut rxn_matrix: HashMap<(String, String), Vec<SpeciesStatus>> = HashMap::new();

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

                rxn_matrix.insert((sp_id.clone(), rxn_id.clone()), Vec::new());

                for reactant in reactants {
                    if sp.id == reactant.species {
                        let stoichiometry = reactant.stoichiometry.unwrap();
                        rxn_matrix
                            .entry((sp_id.clone(), rxn_id.clone()))
                            .and_modify(|v| v.push(SpeciesStatus::Reactant(stoichiometry)));
                    }
                }

                for product in products {
                    if sp.id == product.species {
                        let stoichiometry = product.stoichiometry.unwrap();
                        rxn_matrix
                            .entry((sp_id.clone(), rxn_id.clone()))
                            .and_modify(|v| v.push(SpeciesStatus::Product(stoichiometry)));
                    }
                }
            }
        }

        rxn_matrix
    }
}
