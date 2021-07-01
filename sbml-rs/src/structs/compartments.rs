use super::tag::TagIndex;

#[derive(Debug, Default)]
pub struct ListOfCompartments {
    pub compartments: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Default, Clone)]
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
