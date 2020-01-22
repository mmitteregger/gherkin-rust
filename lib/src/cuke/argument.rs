use cucumber_messages::pickle;

use crate::cuke::{Location, String, Table};

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

impl<'d> From<Argument<'d>> for pickle::Argument {
    fn from(argument: Argument<'d>) -> Self {
        match argument {
            Argument::String(string) => pickle::Argument {
                message: Some(pickle::ArgumentMessage::DocString(pickle::DocString::from(
                    string,
                ))),
            },
            Argument::Table(table) => pickle::Argument {
                message: Some(pickle::ArgumentMessage::DataTable(pickle::Table::from(
                    table,
                ))),
            },
        }
    }
}
