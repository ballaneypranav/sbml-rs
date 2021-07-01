use super::tag::TagIndex;
use std::fmt;

// An SBML Model container
#[derive(Debug, Default)]
pub struct Model {
    pub name: Option<String>,
    pub nodes: Vec<Tag>,
}

impl Model {
    pub fn new(nodes: Vec<Tag>) -> Self {
        Model { name: None, nodes }
    }
}
impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Model")
    }
}
