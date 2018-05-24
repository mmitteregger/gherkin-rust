use std::fmt::Debug;

use erased_serde::Serialize;

pub use self::compiler::*;
pub use self::pickle::*;
pub use self::pickle_cell::*;
pub use self::pickle_location::*;
pub use self::pickle_row::*;
pub use self::pickle_step::*;
pub use self::pickle_string::*;
pub use self::pickle_table::*;
pub use self::pickle_tag::*;

mod compiler;
mod pickle;
mod pickle_cell;
mod pickle_location;
mod pickle_row;
mod pickle_step;
mod pickle_string;
mod pickle_table;
mod pickle_tag;

pub trait Argument: Serialize + Debug + CloneArgument {
    fn get_location(&self) -> &PickleLocation;
}

serialize_trait_object!(Argument);

pub trait CloneArgument {
    fn clone_argument(&self) -> Box<Argument>;
}

impl<T> CloneArgument for T where T: 'static + Argument + Clone,
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
