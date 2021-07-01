use super::compartments::Compartment;
use super::function_definitions::FunctionDefinition;
use super::parameters::Parameter;
use super::reactions::Reaction;
use super::species::Species;
use super::tag::{Tag, TagIndex};
use super::units::UnitDefinition;
use std::fmt;

// An SBML Model container
#[derive(Debug, Default)]
pub struct Model {
    pub name: Option<String>,
    pub nodes: Vec<Tag>,
}

macro_rules! objects_from_list {
    ($parent_type: ident, $parent_field: ident, $node_type: ident, $node_field: ident) => {
        pub fn $parent_field(&self) -> Vec<$node_type> {
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
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Model")
    }
}
