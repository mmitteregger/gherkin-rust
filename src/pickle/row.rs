use pickle::*;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PickleRow {
    cells: Vec<PickleCell>,
}

impl PickleRow {
    pub fn new(cells: Vec<PickleCell>) -> PickleRow {
        PickleRow {
            cells,
        }
    }

    pub fn get_cells(&self)-> &Vec<PickleCell> {
        &self.cells
    }
}
