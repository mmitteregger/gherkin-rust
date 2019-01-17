use pickle::*;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PickleString {
    pub location: PickleLocation,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl PickleString {
    pub fn get_location(&self) -> &PickleLocation {
        &self.location
    }
}

impl AsRef<str> for PickleString {
    fn as_ref(&self) -> &str {
        &self.content
    }
}
