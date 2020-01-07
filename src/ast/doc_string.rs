use crate::ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DocString {
    #[serde(rename = "type")]
    node_type: &'static str,
    pub location: Location,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    pub content: String,
}

impl DocString {
    pub fn new(location: Location, content_type: Option<String>, content: String) -> DocString {
        DocString {
            node_type: "DocString",
            location,
            content_type,
            content,
        }
    }
}
