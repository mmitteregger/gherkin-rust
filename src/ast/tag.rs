use ast::*;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    #[serde(rename = "type")]
    node_type: &'static str,
    location: Location,
    name: String,
}

impl Tag {
    pub fn new(location: Location, name: String) -> Tag {
        Tag {
            node_type: "Tag",
            location,
            name,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub(crate) fn take_name(self) -> String {
        self.name
    }

    pub fn get_location(&self) -> Location {
        self.location
    }
}
