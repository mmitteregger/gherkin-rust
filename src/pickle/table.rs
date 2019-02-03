use pickle::*;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub rows: Vec<Row>,
}

impl Table {
    pub fn get_location(&self) -> &Location {
        &self.rows[0].cells[0].location
    }
}
