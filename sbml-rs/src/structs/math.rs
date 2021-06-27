use super::tag::TagIndex;
pub use mathml_rs::MathNode;
use std::fmt;

#[derive(Debug, Default)]
pub struct MathTag {
    pub nodes: Vec<MathNode>,
    pub parent: Option<TagIndex>,
}

impl fmt::Display for MathTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for node in &self.nodes {
            writeln!(f, "{}", node)?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
impl MathTag {
    pub fn with_nodes(mut self, nodes: Vec<MathNode>) -> Self {
        self.nodes = nodes;
        self
    }

    pub fn with_parent(mut self, parent: TagIndex) -> Self {
        self.parent = Some(parent);
        self
    }
}
