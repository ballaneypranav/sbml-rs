use super::tag::TagIndex;

#[derive(Debug, Default)]
pub struct ListOfSpecies {
    pub species: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Default)]
pub struct Species {
    pub name: Option<String>,
    pub compartment: Option<String>,
    pub parent: Option<TagIndex>,
}
