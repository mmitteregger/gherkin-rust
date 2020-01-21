use std::borrow::Cow;

use cucumber_messages::pickle;

use crate::cuke::Location;

#[derive(Debug, Clone)]
pub struct Cell<'d> {
    pub location: Location,
    pub value: Cow<'d, str>,
}

impl<'d> AsRef<str> for Cell<'d> {
    fn as_ref(&self) -> &str {
        &self.value.as_ref()
    }
}

impl<'d> From<Cell<'d>> for pickle::TableCell {
    fn from(cell: Cell<'d>) -> pickle::TableCell {
        pickle::TableCell {
            value: cell.value.to_string(),
        }
    }
}
