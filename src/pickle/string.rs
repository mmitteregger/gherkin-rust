use pickle::*;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PickleString {
    pub location: PickleLocation,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl Argument for PickleString {
    fn get_location(&self) -> &PickleLocation {
        &self.location
    }
}
