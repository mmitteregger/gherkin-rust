use pickle::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PickleTable {
    rows: Vec<PickleRow>,
}

impl PickleTable {
    pub fn new(rows: Vec<PickleRow>) -> PickleTable {
        PickleTable {
            rows,
        }
    }

    pub fn get_rows(&self) -> &Vec<PickleRow> {
        &self.rows
    }
}

impl Argument for PickleTable {
    fn get_location(&self) -> &PickleLocation {
        &self.rows.get(0).unwrap().get_cells().get(0).unwrap().get_location()
    }
}
