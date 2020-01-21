use std::borrow::Cow;
use std::string::String as StdString;

use cucumber_messages::pickle;

use crate::cuke::{Argument, Location};

#[derive(Debug, Clone)]
pub struct Step<'d> {
    pub id: StdString,
    pub keyword: &'d str,
    pub text: Cow<'d, str>,
    pub argument: Option<Argument<'d>>,
    pub locations: Vec<Location>,
    pub ast_node_ids: Vec<&'d str>,
}

impl<'d> From<Step<'d>> for pickle::Step {
    fn from(step: Step<'d>) -> Self {
        pickle::Step {
            id: step.id,
            text: step.text.to_string(),
            argument: step.argument.map(pickle::Argument::from),
            ast_node_ids: step.ast_node_ids.into_iter().map(str::to_string).collect(),
        }
    }
}
