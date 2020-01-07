pub use self::scenario_definition::*;
pub use self::argument::*;
pub use self::cell::*;
pub use self::compiler::*;
pub use self::location::*;
pub use self::row::*;
pub use self::step::*;
pub use self::string::*;
pub use self::table::*;
pub use self::tag::*;
use crate::ast;
use std::borrow::Cow;

mod scenario_definition;
mod argument;
mod cell;
mod compiler;
mod location;
mod row;
mod step;
mod string;
mod table;
mod tag;

#[derive(Debug)]
pub struct Cuke<'d> {
    pub feature: &'d ast::Feature,
    pub background: Option<&'d ast::Background>,
    pub scenario_definition: ScenarioDefinition<'d>,
    pub name: Cow<'d, str>,
    pub language: &'d str,
    pub background_steps: Vec<Step<'d>>,
    pub scenario_steps: Vec<Step<'d>>,
    pub tags: Vec<Tag<'d>>,
    pub locations: Vec<Location>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_sync<T: Sync>() {}
    fn assert_send<T: Send>() {}

    #[test]
    fn test_send_sync() {
        assert_send::<Cuke>();
        assert_sync::<Cuke>();
    }
}
