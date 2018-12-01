pub use self::argument::*;
pub use self::cell::*;
pub use self::compiler::*;
pub use self::location::*;
pub use self::row::*;
pub use self::step::*;
pub use self::string::*;
pub use self::table::*;
pub use self::tag::*;

mod argument;
mod cell;
mod compiler;
mod location;
mod row;
mod step;
mod string;
mod table;
mod tag;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pickle {
    pub name: String,
    pub language: String,
    pub steps: Vec<PickleStep>,
    pub tags: Vec<PickleTag>,
    pub locations: Vec<PickleLocation>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_sync<T: Sync>() {}
    fn assert_send<T: Send>() {}

    #[test]
    fn test_send_sync() {
        assert_send::<Pickle>();
        assert_sync::<Pickle>();
    }
}
