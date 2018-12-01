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
    argument: Option<Argument>,
}

impl Step {
    pub fn new(
        location: Location,
        keyword: String,
        text: String,
        argument: Option<Argument>,
    ) -> Step {
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

    pub fn get_argument(&self) -> Option<&Argument> {
        self.argument.as_ref()
    }

    pub fn get_location(&self) -> Location {
        self.location
    }
}
