use crate::ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScenarioOutline {
    #[serde(rename = "type")]
    node_type: &'static str,
    pub location: Location,
    pub keyword: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub steps: Vec<Step>,
    pub tags: Vec<Tag>,
    pub examples: Vec<Examples>,
}

impl ScenarioOutline {
    pub fn new(
        location: Location,
        keyword: String,
        name: String,
        description: Option<String>,
        steps: Vec<Step>,
        tags: Vec<Tag>,
        examples: Vec<Examples>,
    ) -> ScenarioOutline {
        ScenarioOutline {
            node_type: "ScenarioOutline",
            location,
            keyword,
            name,
            description,
            steps,
            tags,
            examples,
        }
    }
}
