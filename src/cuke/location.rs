use cucumber_messages::ast;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

impl From<ast::Location> for Location {
    fn from(location: ast::Location) -> Self {
        Location {
            line: location.line,
            column: location.column,
        }
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
