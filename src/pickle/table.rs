use pickle::*;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PickleTable {
    pub rows: Vec<PickleRow>,
}

impl Argument for PickleTable {
    fn get_location(&self) -> &PickleLocation {
        &self.rows[0].cells[0].location
    }
}
