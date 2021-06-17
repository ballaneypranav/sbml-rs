use super::tag::TagIndex;

#[derive(Debug)]
pub struct ListOfCompartments {
    pub compartments: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

impl ListOfCompartments {
    pub fn new() -> Self {
        ListOfCompartments {
            compartments: Vec::new(),
            parent: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct Compartment {
    pub units: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub sbo_term: Option<String>,
    pub spatial_dimensions: Option<f64>,
    pub size: Option<f64>,
    pub constant: Option<bool>,
    pub parent: Option<TagIndex>,
}

impl Compartment {
    pub fn new() -> Self {
        Compartment::default()
    }
}
