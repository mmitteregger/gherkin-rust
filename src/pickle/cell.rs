use pickle::*;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PickleCell {
    pub location: PickleLocation,
    pub value: String,
}

impl AsRef<str> for PickleCell {
    fn as_ref(&self) -> &str {
        &self.value
    }
}
