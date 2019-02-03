use pickle::Location;
use cuke;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub location: Location,
    pub name: String,
}

impl<'d> From<cuke::Tag<'d>> for Tag {
    fn from(cuke_tag: cuke::Tag<'d>) -> Self {
        Tag {
            location: Location::from(cuke_tag.location),
            name: cuke_tag.name.to_string(),
        }
    }
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        &self.name
    }
}
