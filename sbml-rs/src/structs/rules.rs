use crate::{MathTag, Model, Tag, TagIndex};

#[derive(Debug, Default)]
pub struct ListOfRules {
    pub assignment_rules: Vec<TagIndex>,
    pub rate_rules: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Default, Clone)]
pub struct AssignmentRule {
    pub id: Option<String>,
    pub metaid: Option<String>,
    pub variable: Option<String>,
    pub sbo_term: Option<String>,
    pub math: Option<TagIndex>,
    pub parent: Option<TagIndex>,
}

impl AssignmentRule {
    pub fn math_tag(&self, model: &Model) -> Option<MathTag> {
        let mut result = None;
        if let Some(math_tag_idx) = self.math {
            if let Tag::MathTag(math_tag) = &model.nodes[math_tag_idx] {
                result = Some(math_tag.clone());
            }
        }
        result
    }
}

#[derive(Debug, Default, Clone)]
pub struct RateRule {
    pub id: Option<String>,
    pub metaid: Option<String>,
    pub variable: Option<String>,
    pub sbo_term: Option<String>,
    pub math: Option<TagIndex>,
    pub parent: Option<TagIndex>,
}

impl RateRule {
    pub fn math_tag(&self, model: &Model) -> Option<MathTag> {
        let mut result = None;
        if let Some(math_tag_idx) = self.math {
            if let Tag::MathTag(math_tag) = &model.nodes[math_tag_idx] {
                result = Some(math_tag.clone());
            }
        }
        result
    }
}
