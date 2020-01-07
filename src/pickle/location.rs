use serde::Serialize;

use crate::cuke;

#[derive(Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

impl From<cuke::Location> for Location {
    fn from(cuke_location: cuke::Location) -> Self {
        Location {
            line: cuke_location.line,
            column: cuke_location.column,
        }
    }
}
