use super::tag::TagIndex;
use std::fmt;

// An SBML Model container
#[derive(Debug, Default)]
pub struct Model {
    pub name: Option<String>,
    pub list_of_species: Option<TagIndex>,
    pub list_of_reactions: Option<TagIndex>,
    pub list_of_unit_definitions: Option<TagIndex>,
    pub list_of_compartments: Option<TagIndex>,
    pub list_of_parameters: Option<TagIndex>,
    pub list_of_function_definitions: Option<TagIndex>,
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Model")
    }
}
