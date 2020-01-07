use serde::Serialize;

use crate::cuke;
use crate::pickle::{Argument, Location};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    pub text: String,
    pub arguments: Vec<Argument>,
    pub locations: Vec<Location>,
}

impl<'d> From<cuke::Step<'d>> for Step {
    fn from(cuke_step: cuke::Step<'d>) -> Self {
        Step {
            text: cuke_step.text.to_string(),
            arguments: cuke_step.argument.into_iter().map(Argument::from).collect(),
            locations: cuke_step
                .locations
                .into_iter()
                .map(Location::from)
                .collect(),
        }
    }
}
