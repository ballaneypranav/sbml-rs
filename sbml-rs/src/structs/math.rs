//use std::collections::HashMap;

use mathml_rs::MathNode;
use super::tag::TagIndex;

#[derive(Debug)]
pub struct MathTag {
    pub nodes: Vec<MathNode>,
    pub parent: Option<TagIndex>,
}

impl MathTag {
    pub fn new() -> Self {
        MathTag {
            nodes: Vec::new(),
            parent: None,
        }
    }
    pub fn with_nodes(mut self, nodes: Vec<MathNode>) -> Self {
        self.nodes = nodes;
        self
    }

    pub fn with_parent(mut self, parent: TagIndex) -> Self {
        self.parent = Some(parent);
        self
    }
}
