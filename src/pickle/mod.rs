use std::fmt::Debug;

use downcast::Downcast;
use erased_serde::Serialize;

pub use self::cell::*;
pub use self::compiler::*;
pub use self::location::*;
pub use self::row::*;
pub use self::step::*;
pub use self::string::*;
pub use self::table::*;
pub use self::tag::*;

mod cell;
mod compiler;
mod location;
mod row;
mod step;
mod string;
mod table;
mod tag;

pub trait Argument: Serialize + Downcast + Debug + CloneArgument + Send + Sync {
    fn get_location(&self) -> &PickleLocation;
}

serialize_trait_object!(Argument);
impl_downcast!(Argument);

pub trait CloneArgument {
    fn clone_argument(&self) -> Box<Argument>;
}

impl<T> CloneArgument for T
where
    T: 'static + Argument + Clone,
{
    fn clone_argument(&self) -> Box<Argument> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Argument> {
    fn clone(&self) -> Box<Argument> {
        self.clone_argument()
    }
}

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
