use crate::ast::*;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ScenarioDefinition {
    Background(Background),
    Scenario(Scenario),
    ScenarioOutline(ScenarioOutline),
}

impl ScenarioDefinition {
    pub fn get_location(&self) -> Location {
        match self {
            ScenarioDefinition::Background(background) => background.location,
            ScenarioDefinition::Scenario(scenario) => scenario.location,
            ScenarioDefinition::ScenarioOutline(scenario_outline) => scenario_outline.location,
        }
    }

    pub fn get_keyword(&self) -> &String {
        match self {
            ScenarioDefinition::Background(background) => &background.keyword,
            ScenarioDefinition::Scenario(scenario) => &scenario.keyword,
            ScenarioDefinition::ScenarioOutline(scenario_outline) => &scenario_outline.keyword,
        }
    }

    pub fn get_name(&self) -> &String {
        match self {
            ScenarioDefinition::Background(background) => &background.name,
            ScenarioDefinition::Scenario(scenario) => &scenario.name,
            ScenarioDefinition::ScenarioOutline(scenario_outline) => &scenario_outline.name,
        }
    }

    pub fn get_description(&self) -> Option<&String> {
        match self {
            ScenarioDefinition::Background(background) => background.description.as_ref(),
            ScenarioDefinition::Scenario(scenario) => scenario.description.as_ref(),
            ScenarioDefinition::ScenarioOutline(outline) => outline.description.as_ref(),
        }
    }

    pub fn get_steps(&self) -> &Vec<Step> {
        match self {
            ScenarioDefinition::Background(background) => &background.steps,
            ScenarioDefinition::Scenario(scenario) => &scenario.steps,
            ScenarioDefinition::ScenarioOutline(scenario_outline) => &scenario_outline.steps,
        }
    }
}

impl From<Background> for ScenarioDefinition {
    fn from(background: Background) -> Self {
        ScenarioDefinition::Background(background)
    }
}

impl From<Scenario> for ScenarioDefinition {
    fn from(scenario: Scenario) -> Self {
        ScenarioDefinition::Scenario(scenario)
    }
}

impl From<ScenarioOutline> for ScenarioDefinition {
    fn from(scenario_outline: ScenarioOutline) -> Self {
        ScenarioDefinition::ScenarioOutline(scenario_outline)
    }
}
