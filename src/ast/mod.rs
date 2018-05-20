use std::fmt::Debug;

use erased_serde::Serialize;

pub use self::location::*;
pub use self::tag::*;
pub use self::gherkin_document::*;
pub use self::feature::*;
pub use self::comment::*;
pub use self::table_cell::*;
pub use self::table_row::*;
pub use self::step::*;
pub use self::scenario_definition::*;
pub use self::background::*;
pub use self::examples::*;
pub use self::scenario::*;
pub use self::scenario_outline::*;
pub use self::data_table::*;
pub use self::doc_string::*;

mod location;
mod tag;
mod gherkin_document;
mod feature;
mod comment;
mod table_cell;
mod table_row;
mod step;
mod scenario_definition;
mod background;
mod examples;
mod scenario;
mod scenario_outline;
mod data_table;
mod doc_string;

pub trait Node: Serialize + Debug {
    fn get_location(&self) -> Location;
}

serialize_trait_object!(Node);

// TODO: Try to get rid of all the node_type struct fields inside this module
