use ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScenarioOutline {
    #[serde(rename = "type")]
    node_type: &'static str,
    location: Location,
    keyword: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    steps: Vec<Step>,
    tags: Vec<Tag>,
    examples: Vec<Examples>,
}

impl ScenarioOutline {
    pub fn new(location: Location, keyword: String, name: String, description: Option<String>,
            steps: Vec<Step>, tags: Vec<Tag>, examples: Vec<Examples>) -> ScenarioOutline {
        ScenarioOutline {
            node_type: "ScenarioOutline",
            location,
            keyword,
            name,
            description,
            steps,
            tags,
            examples,
        }
    }

    pub fn get_tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn get_examples(&self) -> &Vec<Examples> {
        &self.examples
    }
}

impl Node for ScenarioOutline {
    fn get_location(&self) -> Location {
        self.location
    }
}

impl ScenarioDefinition for ScenarioOutline {
    fn get_keyword(&self) -> &String {
        &self.keyword
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    fn get_steps(&self) -> &Vec<Step> {
        &self.steps
    }
}
