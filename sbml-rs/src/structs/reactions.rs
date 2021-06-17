use super::tag::TagIndex;

#[derive(Debug, Default)]
pub struct ListOfReactions {
    pub reactions: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Default)]
pub struct Reaction {
    pub kinetic_law: Option<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Default)]
pub struct KineticLaw {
    pub math: Option<TagIndex>,
    pub parent: Option<TagIndex>,
}
