use crate::ast::{Location, Scenario, ScenarioOutline, Step};

#[derive(Debug)]
pub enum ScenarioDefinition<'d> {
    Scenario(&'d Scenario),
    ScenarioOutline(&'d ScenarioOutline),
}

impl<'d> ScenarioDefinition<'d> {
    pub fn get_location(&self) -> Location {
        match self {
            ScenarioDefinition::Scenario(scenario) => scenario.location,
            ScenarioDefinition::ScenarioOutline(scenario_outline) => scenario_outline.location,
        }
    }

    pub fn get_keyword(&self) -> &str {
        match self {
            ScenarioDefinition::Scenario(scenario) => &scenario.keyword,
            ScenarioDefinition::ScenarioOutline(scenario_outline) => &scenario_outline.keyword,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            ScenarioDefinition::Scenario(scenario) => &scenario.name,
            ScenarioDefinition::ScenarioOutline(scenario_outline) => &scenario_outline.name,
        }
    }

    pub fn get_description(&self) -> Option<&str> {
        match self {
            ScenarioDefinition::Scenario(scenario) => scenario
                .description
                .as_ref()
                .map(|description| description.as_str()),
            ScenarioDefinition::ScenarioOutline(outline) => outline
                .description
                .as_ref()
                .map(|description| description.as_str()),
        }
    }

    pub fn get_steps(&self) -> &[Step] {
        match self {
            ScenarioDefinition::Scenario(scenario) => &scenario.steps,
            ScenarioDefinition::ScenarioOutline(scenario_outline) => &scenario_outline.steps,
        }
    }
}

impl<'d> From<&'d Scenario> for ScenarioDefinition<'d> {
    fn from(scenario: &'d Scenario) -> Self {
        ScenarioDefinition::Scenario(scenario)
    }
}

impl<'d> From<&'d ScenarioOutline> for ScenarioDefinition<'d> {
    fn from(scenario_outline: &'d ScenarioOutline) -> Self {
        ScenarioDefinition::ScenarioOutline(scenario_outline)
    }
}
