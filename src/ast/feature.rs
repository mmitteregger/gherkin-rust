use ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    #[serde(rename = "type")]
    node_type: &'static str,
    pub location: Location,
    pub tags: Vec<Tag>,
    pub language: String,
    pub keyword: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "children")]
    pub scenario_definitions: Vec<ScenarioDefinition>,
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
            scenario_definitions,
        }
    }
}
