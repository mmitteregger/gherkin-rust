use cuke::{String, Table, Location};

#[derive(Debug, Clone)]
pub enum Argument<'d> {
    String(String<'d>),
    Table(Table<'d>),
}

impl<'d> Argument<'d> {
    pub fn get_location(&self) -> Location {
        match self {
            Argument::String(string) => string.get_location(),
            Argument::Table(table) => table.get_location(),
        }
    }
}
