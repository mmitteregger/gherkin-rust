use ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    #[serde(rename = "type")]
    node_type: &'static str,
    location: Location,
    text: String,
}

impl Comment {
    pub fn new(location: Location, text: String) -> Comment {
        Comment {
            node_type: "Comment",
            location,
            text,
        }
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }
}

impl Node for Comment {
    fn get_location(&self) -> Location {
        self.location
    }
}
