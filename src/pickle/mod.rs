use std::fmt::Debug;

use erased_serde::Serialize;

pub use self::compiler::*;
pub use self::cell::*;
pub use self::location::*;
pub use self::row::*;
pub use self::step::*;
pub use self::string::*;
pub use self::table::*;
pub use self::tag::*;

mod compiler;
mod cell;
mod location;
mod row;
mod step;
mod string;
mod table;
mod tag;

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

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pickle {
    name: String,
    language: String,
    steps: Vec<PickleStep>,
    tags: Vec<PickleTag>,
    locations: Vec<PickleLocation>,
}

impl Pickle {
    pub fn new(name: String, language: String, steps: Vec<PickleStep>, tags: Vec<PickleTag>,
               locations: Vec<PickleLocation>) -> Pickle {
        Pickle {
            name,
            language,
            steps,
            tags,
            locations,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_language(&self) -> &String {
        &self.language
    }

    pub fn get_steps(&self) -> &Vec<PickleStep> {
        &self.steps
    }

    pub fn get_locations(&self) -> &Vec<PickleLocation> {
        &self.locations
    }

    pub fn get_tags(&self) -> &Vec<PickleTag> {
        &self.tags
    }
}
