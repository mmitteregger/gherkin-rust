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

pub trait Argument: Serialize + Debug {
    fn get_location(&self) -> &PickleLocation;
}

serialize_trait_object!(Argument);
