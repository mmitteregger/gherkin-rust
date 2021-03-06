use std::fmt;

use cucumber_messages::ast;
use serde::Serialize;

#[derive(Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

impl Location {
    pub fn new(line: u32, column: u32) -> Location {
        Location { line, column }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({}:{})", self.line, self.column)
    }
}

impl From<Location> for ast::Location {
    fn from(location: Location) -> Self {
        ast::Location {
            line: location.line,
            column: location.column,
        }
    }
}
