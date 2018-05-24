use ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Scenario {
    #[serde(rename = "type")]
    node_type: &'static str,
    location: Location,
    keyword: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    steps: Vec<Step>,
    tags: Vec<Tag>,
}

impl Scenario {
    pub fn new(location: Location, keyword: String, name: String, description: Option<String>,
            steps: Vec<Step>, tags: Vec<Tag>) -> Scenario {
        Scenario {
            node_type: "Scenario",
            location,
            keyword,
            name,
            description,
            steps,
            tags,
        }
    }

    pub fn get_tags(&self) -> &Vec<Tag> {
        &self.tags
    }
}

impl Node for Scenario {
    fn get_location(&self) -> Location {
        self.location
    }
}

impl ScenarioDefinition for Scenario {
    fn get_keyword(&self) -> &String {
        &self.keyword
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_description(&self) -> &Option<String> {
        &self.description
    }

    fn get_steps(&self) -> &Vec<Step> {
        &self.steps
    }
}
