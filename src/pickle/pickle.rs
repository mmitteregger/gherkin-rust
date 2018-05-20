use pickle::*;

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
