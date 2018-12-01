use ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    #[serde(rename = "type")]
    node_type: &'static str,
    location: Location,
    tags: Vec<Tag>,
    language: String,
    keyword: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "children")]
    scenario_definitions: Vec<ScenarioDefinition>,
}

impl Feature {
    pub fn new(
        location: Location,
        tags: Vec<Tag>,
        language: String,
        keyword: String,
        name: String,
        description: Option<String>,
        scenario_definitions: Vec<ScenarioDefinition>,
    ) -> Feature {
        Feature {
            node_type: "Feature",
            location,
            tags,
            language,
            keyword,
            name,
            description,
            scenario_definitions: scenario_definitions,
        }
    }

    pub fn get_tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn get_language(&self) -> &String {
        &self.language
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

    pub fn get_scenario_definitions(&self) -> &Vec<ScenarioDefinition> {
        &self.scenario_definitions
    }

    pub fn get_location(&self) -> Location {
        self.location
    }
}
