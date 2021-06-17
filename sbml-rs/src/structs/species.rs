use super::tag::TagIndex;

#[derive(Debug, Default)]
pub struct ListOfSpecies {
    pub species: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Default)]
pub struct Species {
    pub id: Option<String>,
    pub name: Option<String>,
    pub meta_id: Option<String>,
    pub sbo_term: Option<String>,
    pub compartment: Option<String>,
    pub initial_concentration: Option<f64>,
    pub initial_amount: Option<f64>,
    pub substance_units: Option<String>,
    pub has_only_substance_units: Option<bool>,
    pub boundary_condition: Option<bool>,
    pub constant: Option<bool>,
    pub conversion_factor: Option<String>,
    pub parent: Option<TagIndex>,
}
