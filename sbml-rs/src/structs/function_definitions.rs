use mathml_rs::{evaluate_lambda, MathNode};

use super::math::MathTag;
use super::model::Model;
use super::tag::{Tag, TagIndex};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct ListOfFunctionDefinitions {
    pub function_definitions: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Clone, Debug, Default)]
pub struct FunctionDefinition {
    pub id: Option<String>,
    pub name: Option<String>,
    pub sbo_term: Option<String>,
    pub math: Option<TagIndex>,
    pub parent: Option<TagIndex>,
}

impl FunctionDefinition {
    pub fn math_tag(&self, model: &Model) -> Option<MathTag> {
        let mut result = None;
        if let Some(math_tag_idx) = self.math {
            if let Tag::MathTag(math_tag) = &model.nodes[math_tag_idx] {
                result = Some(math_tag.clone());
            }
        }
        result
    }

    pub fn evaluate(
        &self,
        model: &Model,
        argument_values: &[f64],
        functions: &HashMap<String, Vec<MathNode>>,
    ) -> Result<f64, String> {
        let math_tag = self.math_tag(model).unwrap();
        evaluate_lambda(&math_tag.nodes, 0, &argument_values, functions)
    }
}
