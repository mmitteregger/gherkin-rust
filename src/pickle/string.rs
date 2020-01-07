use crate::pickle::Location;
use crate::cuke;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct String {
    pub location: Location,
    pub content: ::std::string::String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<::std::string::String>,
}

impl<'d> From<cuke::String<'d>> for String {
    fn from(cuke_string: cuke::String<'d>) -> Self {
        String {
            location: Location::from(cuke_string.location),
            content: cuke_string.content.to_string(),
            content_type: cuke_string.content_type.map(|content_type| content_type.to_string()),
        }
    }
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
