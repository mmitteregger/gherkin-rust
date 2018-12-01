use ast::*;

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
            ScenarioDefinition::Background(background) => background.get_location(),
            ScenarioDefinition::Scenario(scenario) => scenario.get_location(),
            ScenarioDefinition::ScenarioOutline(scenario_outline) => scenario_outline.get_location(),
        }
    }

    pub fn get_keyword(&self) -> &String {
        match self {
            ScenarioDefinition::Background(background) => background.get_keyword(),
            ScenarioDefinition::Scenario(scenario) => scenario.get_keyword(),
            ScenarioDefinition::ScenarioOutline(scenario_outline) => scenario_outline.get_keyword(),
        }
    }

    pub fn get_name(&self) -> &String {
        match self {
            ScenarioDefinition::Background(background) => background.get_name(),
            ScenarioDefinition::Scenario(scenario) => scenario.get_name(),
            ScenarioDefinition::ScenarioOutline(scenario_outline) => scenario_outline.get_name(),
        }
    }

    pub fn get_description(&self) -> Option<&String> {
        match self {
            ScenarioDefinition::Background(background) => background.get_description(),
            ScenarioDefinition::Scenario(scenario) => scenario.get_description(),
            ScenarioDefinition::ScenarioOutline(scenario_outline) => scenario_outline.get_description(),
        }
    }

    pub fn get_steps(&self) -> &Vec<Step> {
        match self {
            ScenarioDefinition::Background(background) => background.get_steps(),
            ScenarioDefinition::Scenario(scenario) => scenario.get_steps(),
            ScenarioDefinition::ScenarioOutline(scenario_outline) => scenario_outline.get_steps(),
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
