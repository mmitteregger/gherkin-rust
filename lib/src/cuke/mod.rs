pub use self::argument::*;
pub use self::cell::*;
pub use self::compiler::*;
pub use self::location::*;
pub use self::row::*;
pub use self::step::*;
pub use self::string::*;
pub use self::table::*;
pub use self::tag::*;

use std::string::String as StdString;
use std::borrow::Cow;

use cucumber_messages::ast;
use cucumber_messages::pickle;

mod argument;
mod cell;
mod compiler;
mod location;
mod row;
mod step;
mod string;
mod table;
mod tag;
//mod rule;

#[derive(Debug)]
pub struct Cuke<'d> {
    pub id: StdString,
    pub uri: &'d str,
    pub feature: &'d ast::Feature,
    pub feature_background: Option<&'d ast::Background>,
    pub rule: Option<&'d ast::Rule>,
    pub rule_background: Option<&'d ast::Background>,
    pub scenario: &'d ast::Scenario,
    pub name: Cow<'d, str>,
    pub language: &'d str,
    pub feature_background_steps: Vec<Step<'d>>,
    pub rule_background_steps: Vec<Step<'d>>,
    pub scenario_steps: Vec<Step<'d>>,
    pub tags: Vec<Tag<'d>>,
    pub locations: Vec<Location>,
    pub ast_node_ids: Vec<&'d str>,
}

impl<'d> From<Cuke<'d>> for pickle::Pickle {
    fn from(cuke: Cuke<'d>) -> Self {
        let steps_capacity = cuke.feature_background_steps.len()
            + cuke.rule_background_steps.len()
            + cuke.scenario_steps.len();
        let mut steps = Vec::with_capacity(steps_capacity);
        cuke.feature_background_steps
            .into_iter()
            .map(pickle::Step::from)
            .for_each(|step| steps.push(step));
        cuke.rule_background_steps
            .into_iter()
            .map(pickle::Step::from)
            .for_each(|step| steps.push(step));
        cuke.scenario_steps
            .into_iter()
            .map(pickle::Step::from)
            .for_each(|step| steps.push(step));

        pickle::Pickle {
            id: cuke.id,
            uri: cuke.uri.to_string(),
            name: cuke.name.to_string(),
            language: cuke.language.to_string(),
            steps,
            tags: cuke.tags.into_iter().map(pickle::Tag::from).collect(),
            ast_node_ids: cuke.ast_node_ids.into_iter().map(str::to_string).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_sync<T: Sync>() {}
    fn assert_send<T: Send>() {}

    #[test]
    fn test_send_sync() {
        assert_send::<Cuke<'_>>();
        assert_sync::<Cuke<'_>>();
    }
}
