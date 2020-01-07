use serde::Serialize;

use crate::ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Scenario {
    #[serde(rename = "type")]
    node_type: &'static str,
    pub location: Location,
    pub keyword: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub steps: Vec<Step>,
    pub tags: Vec<Tag>,
}

impl Scenario {
    pub fn new(
        location: Location,
        keyword: String,
        name: String,
        description: Option<String>,
        steps: Vec<Step>,
        tags: Vec<Tag>,
    ) -> Scenario {
        Scenario {
            node_type: "Scenario",
            location,
            keyword,
            name,
            description,
            steps,
            tags,
        }
    }
}
