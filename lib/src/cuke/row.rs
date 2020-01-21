use cucumber_messages::pickle;

use crate::cuke::Cell;

#[derive(Debug, Clone)]
pub struct Row<'d> {
    pub cells: Vec<Cell<'d>>,
}

impl<'d> From<Row<'d>> for pickle::TableRow {
    fn from(row: Row<'d>) -> pickle::TableRow {
        pickle::TableRow {
            cells: row.cells.into_iter().map(pickle::TableCell::from).collect(),
        }
    }
}
