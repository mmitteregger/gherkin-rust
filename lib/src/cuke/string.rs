use std::borrow::Cow;

use cucumber_messages::pickle;

use crate::cuke::Location;

#[derive(Debug, Clone)]
pub struct String<'d> {
    pub location: Location,
    pub content: Cow<'d, str>,
    pub media_type: Cow<'d, str>,
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

impl<'d> From<String<'d>> for pickle::DocString {
    fn from(string: String<'d>) -> pickle::DocString {
        pickle::DocString {
            content: string.content.to_string(),
            media_type: string.media_type.into_owned(),
        }
    }
}
