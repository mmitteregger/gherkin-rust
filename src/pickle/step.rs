use pickle::*;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PickleStep {
    text: String,
    arguments: Vec<Box<Argument>>,
    locations: Vec<PickleLocation>,
}

impl PickleStep {
    pub fn new(
        text: String,
        arguments: Vec<Box<Argument>>,
        locations: Vec<PickleLocation>,
    ) -> PickleStep {
        PickleStep {
            text,
            arguments,
            locations,
        }
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn get_locations(&self) -> &Vec<PickleLocation> {
        &self.locations
    }

    pub fn get_argument(&self) -> &Vec<Box<Argument>> {
        &self.arguments
    }
}
