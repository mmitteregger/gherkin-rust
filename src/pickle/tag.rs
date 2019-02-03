use pickle::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PickleTag {
    pub location: PickleLocation,
    pub name: String,
}

impl AsRef<str> for PickleTag {
    fn as_ref(&self) -> &str {
        &self.name
    }
}
