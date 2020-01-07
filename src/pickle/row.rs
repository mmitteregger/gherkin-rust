use crate::pickle::Cell;
use crate::cuke;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub cells: Vec<Cell>,
}

impl<'d> From<cuke::Row<'d>> for Row {
    fn from(cuke_row: cuke::Row<'d>) -> Self {
        Row {
            cells: cuke_row.cells.into_iter().map(Cell::from).collect(),
        }
    }
}
