use cucumber_messages::pickle;

use crate::cuke::{Location, Row};

#[derive(Debug, Clone)]
pub struct Table<'d> {
    pub rows: Vec<Row<'d>>,
}

impl<'d> Table<'d> {
    pub fn get_location(&self) -> Location {
        self.rows[0].cells[0].location
    }
}

impl<'d> From<Table<'d>> for pickle::Table {
    fn from(table: Table<'d>) -> pickle::Table {
        pickle::Table {
            rows: table.rows.into_iter().map(pickle::TableRow::from).collect(),
        }
    }
}
