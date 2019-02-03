use pickle::Location;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub location: Location,
    pub name: String,
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        &self.name
    }
}
