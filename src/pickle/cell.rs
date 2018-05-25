use pickle::*;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PickleCell {
    location: PickleLocation,
    value: String,
}

impl PickleCell {
    pub fn new(location: PickleLocation, value: String) -> PickleCell {
        PickleCell { location, value }
    }

    pub fn get_location(&self) -> &PickleLocation {
        &self.location
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}
