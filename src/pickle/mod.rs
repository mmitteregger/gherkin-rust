pub use self::argument::*;
pub use self::cell::*;
pub use self::location::*;
pub use self::row::*;
pub use self::step::*;
pub use self::string::*;
pub use self::table::*;
pub use self::tag::*;
use crate::cuke;
use std::string::String as StdString;

mod argument;
mod cell;
mod location;
mod row;
mod step;
mod string;
mod table;
mod tag;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pickle {
    pub name: StdString,
    pub language: StdString,
    pub steps: Vec<Step>,
    pub tags: Vec<Tag>,
    pub locations: Vec<Location>,
}

impl<'d> From<cuke::Cuke<'d>> for Pickle {
    fn from(cuke: cuke::Cuke<'d>) -> Self {
        let steps_capacity = cuke.background_steps.len() + cuke.scenario_steps.len();
        let mut steps = Vec::with_capacity(steps_capacity);
        cuke.background_steps.into_iter().map(Step::from).for_each(|step| steps.push(step));
        cuke.scenario_steps.into_iter().map(Step::from).for_each(|step| steps.push(step));

        Pickle {
            name: cuke.name.to_string(),
            language: cuke.language.to_string(),
            steps,
            tags: cuke.tags.into_iter().map(Tag::from).collect(),
            locations: cuke.locations.into_iter().map(Location::from).collect(),
        }
    }
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
