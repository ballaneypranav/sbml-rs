use super::tag::TagIndex;

// An SBML Model container
#[derive(Debug, Default)]
pub struct Model {
    pub name: Option<String>,
    pub list_of_species: Option<TagIndex>,
    pub list_of_reactions: Option<TagIndex>,
    pub list_of_unit_definitions: Option<TagIndex>,
    pub list_of_compartments: Option<TagIndex>,
}
