use pickle::*;

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Argument {
    String(PickleString),
    Table(PickleTable),
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}

impl Argument {
    pub fn get_location(&self) -> &PickleLocation {
        match self {
            Argument::String(pickle_string) => pickle_string.get_location(),
            Argument::Table(pickle_table) => pickle_table.get_location(),
            Argument::__Nonexhaustive => unreachable!(),
        }
    }
}
