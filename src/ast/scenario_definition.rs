use std::fmt::Debug;

use erased_serde::Serialize;
use downcast::Downcast;

use ast::*;

pub trait ScenarioDefinition: Serialize + Downcast + Debug + Send + Sync {
    fn get_location(&self) -> Location;

    fn get_keyword(&self) -> &String;

    fn get_name(&self) -> &String;

    fn get_description(&self) -> Option<&String>;

    fn get_steps(&self) -> &Vec<Step>;
}

serialize_trait_object!(ScenarioDefinition);
impl_downcast!(ScenarioDefinition);
