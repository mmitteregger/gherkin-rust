use ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    #[serde(rename = "type")]
    node_type: &'static str,
    location: Location,
    keyword: String,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    argument: Option<Box<Node>>,
}

impl Step {
    pub fn new(location: Location, keyword: String, text: String, argument: Option<Box<Node>>) -> Step {
        Step {
            node_type: "Step",
            location,
            keyword,
            text,
            argument,
        }
    }

    pub fn get_keyword(&self) -> &String {
        &self.keyword
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    #[allow(unknown_lints, borrowed_box)] // required for downcasting to a concrete type
    pub fn get_argument(&self) -> Option<&Box<Node>> {
        self.argument.as_ref()
    }
}

impl Node for Step {
    fn get_location(&self) -> Location {
        self.location
    }
}
