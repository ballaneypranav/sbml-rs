use super::tag::TagIndex;

#[derive(Debug, Default)]
pub struct ListOfReactions {
    pub reactions: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Clone, Default)]
pub struct Reaction {
    pub id: Option<String>,
    pub list_of_reactants: Option<TagIndex>,
    pub list_of_products: Option<TagIndex>,
    pub reversible: Option<bool>,
    pub kinetic_law: Option<TagIndex>,
    pub compartment: Option<String>,
    pub name: Option<String>,
    pub sbo_term: Option<String>,
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Default)]
pub struct ListOfReactants {
    pub species_references: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Default)]
pub struct ListOfProducts {
    pub species_references: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Default)]
pub struct SpeciesReference {
    pub id: Option<String>,
    pub name: Option<String>,
    pub species: Option<String>,
    pub constant: Option<bool>,
    pub sbo_term: Option<String>,
    pub stoichiometry: Option<f64>,
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Default)]
pub struct KineticLaw {
    pub math: Option<TagIndex>,
    pub parent: Option<TagIndex>,
}
