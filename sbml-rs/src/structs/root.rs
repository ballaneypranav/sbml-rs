use super::tag::TagIndex;
use std::fmt;

// SBML Model Root
#[derive(Clone, Debug, Default)]
pub struct Root {
    pub list_of_species: Option<TagIndex>,
    pub list_of_reactions: Option<TagIndex>,
    pub list_of_unit_definitions: Option<TagIndex>,
    pub list_of_compartments: Option<TagIndex>,
    pub list_of_parameters: Option<TagIndex>,
    pub list_of_function_definitions: Option<TagIndex>,
    pub list_of_initial_assignments: Option<TagIndex>,
    pub list_of_rules: Option<TagIndex>,
}

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Root")
    }
}
