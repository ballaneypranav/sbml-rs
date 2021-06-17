use super::tag::TagIndex;

// An SBML Model container
#[derive(Debug)]
pub struct Model {
    pub name: Option<String>,
    pub list_of_species: Option<TagIndex>,
    pub list_of_reactions: Option<TagIndex>,
    pub list_of_unit_definitions: Option<TagIndex>,
    pub list_of_compartments: Option<TagIndex>,
}
impl Model {
    // returns a new SBML model
    pub fn new() -> Model {
        return Model {
            name: None,
            list_of_species: None,
            list_of_reactions: None,
            list_of_unit_definitions: None,
            list_of_compartments: None,
        };
    }
}
