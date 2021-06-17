use super::tag::TagIndex;

#[derive(Debug)]
pub struct ListOfReactions {
    pub reactions: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

impl ListOfReactions {
    pub fn new() -> Self {
        return ListOfReactions {
            reactions: Vec::new(),
            parent: None,
        };
    }
}

#[derive(Debug)]
pub struct Reaction {
    pub kinetic_law: Option<TagIndex>,
    pub parent: Option<TagIndex>,
}

impl Reaction {
    pub fn new() -> Self {
        return Reaction {
            kinetic_law: None,
            parent: None,
        };
    }
}

#[derive(Debug)]
pub struct KineticLaw {
    pub math: Option<TagIndex>,
    pub parent: Option<TagIndex>,
}

impl KineticLaw {
    pub fn new() -> Self {
        return KineticLaw {
            math: None,
            parent: None,
        };
    }
}

