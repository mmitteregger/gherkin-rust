use serde::Serialize;

use crate::ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Background {
    #[serde(rename = "type")]
    node_type: &'static str,
    pub location: Location,
    pub keyword: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub steps: Vec<Step>,
}

impl Background {
    pub fn new(
        location: Location,
        keyword: String,
        name: String,
        description: Option<String>,
        steps: Vec<Step>,
    ) -> Background {
        Background {
            node_type: "Background",
            location,
            keyword,
            name,
            description,
            steps,
        }
    }
}
