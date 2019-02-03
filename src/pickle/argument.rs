use pickle::{String, Table, Location};
use cuke;

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Argument {
    String(String),
    Table(Table),
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}

impl<'d> From<cuke::Argument<'d>> for Argument {
    fn from(cuke_argument: cuke::Argument<'d>) -> Self {
        match cuke_argument {
            cuke::Argument::String(string) => Argument::String(String::from(string)),
            cuke::Argument::Table(table) => Argument::Table(Table::from(table)),
        }
    }
}

impl Argument {
    pub fn get_location(&self) -> &Location {
        match self {
            Argument::String(string) => string.get_location(),
            Argument::Table(table) => table.get_location(),
            Argument::__Nonexhaustive => unreachable!(),
        }
    }
}
