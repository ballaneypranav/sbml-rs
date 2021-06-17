use super::tag::TagIndex;

#[derive(Debug)]
pub struct ListOfSpecies {
    pub species: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

impl ListOfSpecies {
    pub fn new() -> Self {
        return ListOfSpecies {
            species: Vec::new(),
            parent: None,
        };
    }
}

#[derive(Debug)]
pub struct Species {
    pub name: Option<String>,
    pub compartment: Option<String>,
    pub parent: Option<TagIndex>,
}

impl Species {
    pub fn new() -> Self {
        return Species {
            name: None,
            compartment: None,
            parent: None,
        };
    }
}
