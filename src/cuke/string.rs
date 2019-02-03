use cuke::Location;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct String<'d> {
    pub location: Location,
    pub content: Cow<'d, str>,
    pub content_type: Option<Cow<'d, str>>,
}

impl<'d> String<'d> {
    pub fn get_location(&self) -> Location {
        self.location
    }
}

impl<'d> AsRef<str> for String<'d> {
    fn as_ref(&self) -> &str {
        self.content.as_ref()
    }
}
