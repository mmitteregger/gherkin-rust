use ast::*;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    #[serde(rename = "type")]
    node_type: &'static str,
    pub location: Location,
    pub name: String,
}

impl Tag {
    pub fn new(location: Location, name: String) -> Tag {
        Tag {
            node_type: "Tag",
            location,
            name,
        }
    }
}
