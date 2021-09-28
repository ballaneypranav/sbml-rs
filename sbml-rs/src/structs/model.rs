use std::collections::HashMap;

use crate::{
    AssignmentRule, Compartment, FunctionDefinition, InitialAssignment, MathNode, MathTag,
    Parameter, RateRule, Reaction, Species, SpeciesReference, Tag, UnitDefinition,
};

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
}
