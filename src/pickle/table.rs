use pickle::*;

#[derive(Serialize, Debug, Clone)]
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
        &self.rows[0].get_cells()[0].get_location()
    }
}
