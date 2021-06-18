use super::tag::TagIndex;
use mathml_rs::MathNode;

#[derive(Debug, Default)]
pub struct MathTag {
    pub nodes: Vec<MathNode>,
    pub parent: Option<TagIndex>,
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
