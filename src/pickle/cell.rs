use serde::Serialize;

use crate::pickle::Location;
use crate::cuke;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    pub location: Location,
    pub value: String,
}

impl<'d> From<cuke::Cell<'d>> for Cell {
    fn from(cuke_cell: cuke::Cell<'d>) -> Self {
        Cell {
            location: Location::from(cuke_cell.location),
            value: cuke_cell.value.to_string(),
        }
    }
}

impl AsRef<str> for Cell {
    fn as_ref(&self) -> &str {
        &self.value
    }
}
