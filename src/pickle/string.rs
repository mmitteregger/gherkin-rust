use pickle::*;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PickleString {
    location: PickleLocation,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<String>,
}

impl PickleString {
    pub fn new(location: PickleLocation, content: String, content_type: Option<String>) -> PickleString {
        PickleString {
            location,
            content,
            content_type,
        }
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_content_type(&self) -> Option<&String> {
        self.content_type.as_ref()
    }
}

impl Argument for PickleString {
    fn get_location(&self) -> &PickleLocation {
        &self.location
    }
}
