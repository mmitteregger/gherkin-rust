use pickle::Location;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    pub location: Location,
    pub value: String,
}

impl AsRef<str> for Cell {
    fn as_ref(&self) -> &str {
        &self.value
    }
}
