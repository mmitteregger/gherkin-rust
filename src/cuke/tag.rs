use crate::ast;
use crate::cuke::Location;

#[derive(Debug)]
pub struct Tag<'d> {
    pub location: Location,
    pub name: &'d str,
}

impl<'d> From<&'d ast::Tag> for Tag<'d> {
    fn from(tag: &'d ast::Tag) -> Self {
        Tag {
            location: Location::from(tag.location),
            name: &tag.name,
        }
    }
}

impl<'d> AsRef<str> for Tag<'d> {
    fn as_ref(&self) -> &str {
        &self.name
    }
}
