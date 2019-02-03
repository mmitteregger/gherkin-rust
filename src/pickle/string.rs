use pickle::Location;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct String {
    pub location: Location,
    pub content: ::std::string::String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<::std::string::String>,
}

impl String {
    pub fn get_location(&self) -> &Location {
        &self.location
    }
}

impl AsRef<str> for String {
    fn as_ref(&self) -> &str {
        &self.content
    }
}
