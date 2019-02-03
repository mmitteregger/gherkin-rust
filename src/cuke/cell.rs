use cuke::Location;
use std::borrow::Cow;

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
