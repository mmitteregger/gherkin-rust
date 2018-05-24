use ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DocString {
    #[serde(rename = "type")]
    node_type: &'static str,
    location: Location,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<String>,
    content: String,
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

    pub fn get_content_type(&self) -> &Option<String> {
        &self.content_type
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }
}

impl Node for DocString {
    fn get_location(&self) -> Location {
        self.location
    }
}
