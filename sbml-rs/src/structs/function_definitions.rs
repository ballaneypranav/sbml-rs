use super::tag::TagIndex;

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
