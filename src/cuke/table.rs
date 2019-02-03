use cuke::{Row, Location};

#[derive(Debug, Clone)]
pub struct Table<'d> {
    pub rows: Vec<Row<'d>>,
}

impl<'d> Table<'d> {
    pub fn get_location(&self) -> Location {
        self.rows[0].cells[0].location
    }
}
