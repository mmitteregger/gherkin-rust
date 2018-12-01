use ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Background {
    #[serde(rename = "type")]
    node_type: &'static str,
    location: Location,
    keyword: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    steps: Vec<Step>,
}

impl Background {
    pub fn new(
        location: Location,
        keyword: String,
        name: String,
        description: Option<String>,
        steps: Vec<Step>,
    ) -> Background {
        Background {
            node_type: "Background",
            location,
            keyword,
            name,
            description,
            steps,
        }
    }

    pub fn get_location(&self) -> Location {
        self.location
    }

    pub fn get_keyword(&self) -> &String {
        &self.keyword
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn get_steps(&self) -> &Vec<Step> {
        &self.steps
    }
}
