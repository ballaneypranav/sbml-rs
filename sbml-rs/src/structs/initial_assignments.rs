use crate::{MathTag, Model, Tag, TagIndex};

#[derive(Clone, Debug, Default)]
pub struct ListOfInitialAssignments {
    pub initial_assignments: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Default, Clone)]
pub struct InitialAssignment {
    pub id: Option<String>,
    pub symbol: Option<String>,
    pub sbo_term: Option<String>,
    pub math: Option<TagIndex>,
    pub parent: Option<TagIndex>,
}

impl InitialAssignment {
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
