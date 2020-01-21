use cucumber_messages::ast;
use cucumber_messages::pickle;

use crate::cuke::Location;

#[derive(Debug)]
pub struct Tag<'d> {
    pub name: &'d str,
    pub location: Location,
    pub ast_node_id: &'d str,
}

impl<'d> From<&'d ast::Tag> for Tag<'d> {
    fn from(tag: &'d ast::Tag) -> Self {
        Tag {
            name: &tag.name,
            location: Location::from(tag.location.unwrap()),
            ast_node_id: tag.id.as_str(),
        }
    }
}

impl<'d> From<Tag<'d>> for pickle::Tag {
    fn from(tag: Tag<'d>) -> pickle::Tag {
        pickle::Tag {
            name: tag.name.to_string(),
            ast_node_id: tag.ast_node_id.to_string(),
        }
    }
}

impl<'d> AsRef<str> for Tag<'d> {
    fn as_ref(&self) -> &str {
        &self.name
    }
}
