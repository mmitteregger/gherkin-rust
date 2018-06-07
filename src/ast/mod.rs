use std::fmt::Debug;

use downcast::Downcast;
use erased_serde::Serialize;

pub use self::background::*;
pub use self::comment::*;
pub use self::data_table::*;
pub use self::doc_string::*;
pub use self::examples::*;
pub use self::feature::*;
pub use self::gherkin_document::*;
pub use self::location::*;
pub use self::scenario::*;
pub use self::scenario_definition::*;
pub use self::scenario_outline::*;
pub use self::step::*;
pub use self::table_cell::*;
pub use self::table_row::*;
pub use self::tag::*;

mod background;
mod comment;
mod data_table;
mod doc_string;
mod examples;
mod feature;
mod gherkin_document;
mod location;
mod scenario;
mod scenario_definition;
mod scenario_outline;
mod step;
mod table_cell;
mod table_row;
mod tag;

pub trait Node: Serialize + Downcast + Debug + Send + Sync {
    fn get_location(&self) -> Location;
}

serialize_trait_object!(Node);
impl_downcast!(Node);
