use event::*;
use pickle::Pickle;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PickleEvent {
    #[serde(rename = "type")]
    event_type: &'static str,
    uri: String,
    pickle: Pickle,
}

impl PickleEvent {
    pub fn new(uri: String, pickle: Pickle) -> PickleEvent {
        PickleEvent {
            event_type: "pickle",
            uri,
            pickle,
        }
    }
}

impl CucumberEvent for PickleEvent {}
