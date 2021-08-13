use super::tag::TagIndex;
use mathml_rs::evaluate_node;
pub use mathml_rs::MathNode;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, Default)]
pub struct MathTag {
    pub nodes: Vec<MathNode>,
    pub parent: Option<TagIndex>,
}

impl fmt::Display for MathTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut count = 0;
        for node in &self.nodes {
            writeln!(f, "{:2}: {}", count, node)?;
            count += 1;
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

    pub fn evaluate(
        &self,
        assignments: &HashMap<String, f64>,
        functions: &HashMap<String, Vec<MathNode>>,
    ) -> Result<f64, String> {
        evaluate_node(&self.nodes, 0, assignments, functions)
    }
}
