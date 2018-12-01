use pickle::*;

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum PickleArgument {
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

impl PickleArgument {
    pub fn get_location(&self) -> &PickleLocation {
        match self {
            PickleArgument::String(pickle_string) => pickle_string.get_location(),
            PickleArgument::Table(pickle_table) => pickle_table.get_location(),
            PickleArgument::__Nonexhaustive => unreachable!(),
        }
    }
}
