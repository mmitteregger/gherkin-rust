use pickle::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PickleTag {
    location: PickleLocation,
    name: String,
}

impl PickleTag {
    pub fn new(location: PickleLocation, name: String) -> PickleTag {
        PickleTag {
            location,
            name,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
