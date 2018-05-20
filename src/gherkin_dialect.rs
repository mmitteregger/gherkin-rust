use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct GherkinDialect {
    #[serde(skip)]
    pub(crate) language: String,
    pub(crate) name: String,
    #[serde(rename = "native")]
    pub(crate) native_name: String,
    #[serde(rename = "feature")]
    feature_keywords: Vec<String>,
    #[serde(rename = "scenario")]
    scenario_keywords: Vec<String>,
    #[serde(rename = "background")]
    background_keywords: Vec<String>,
    #[serde(rename = "scenarioOutline")]
    scenario_outline_keywords: Vec<String>,
    #[serde(rename = "examples")]
    examples_keywords: Vec<String>,
    #[serde(rename = "given")]
    given_keywords: Vec<String>,
    #[serde(rename = "when")]
    when_keywords: Vec<String>,
    #[serde(rename = "then")]
    then_keywords: Vec<String>,
    #[serde(rename = "and")]
    and_keywords: Vec<String>,
    #[serde(rename = "but")]
    but_keywords: Vec<String>,
    #[serde(skip)]
    pub(crate) step_keywords: Vec<String>,
}

impl ::std::fmt::Debug for GherkinDialect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "GherkinDialect {{ language: \"{}\", ... }}", self.language)
    }
}

fn get_keywords(keywords: &mut HashMap<String, Vec<String>>, key: &str) -> Vec<String> {
    keywords.remove(key).unwrap_or(Vec::new())
}

impl GherkinDialect {
    pub fn new(language: String, name: String, native_name: String,
            mut keywords: HashMap<String, Vec<String>>) -> GherkinDialect {
        let feature_keywords = get_keywords(&mut keywords, "feature");
        let scenario_keywords = get_keywords(&mut keywords, "scenario");
        let background_keywords = get_keywords(&mut keywords, "background");
        let scenario_outline_keywords = get_keywords(&mut keywords, "scenarioOutline");
        let examples_keywords = get_keywords(&mut keywords, "examples");
        let given_keywords = get_keywords(&mut keywords, "given");
        let when_keywords = get_keywords(&mut keywords, "when");
        let then_keywords = get_keywords(&mut keywords, "then");
        let and_keywords = get_keywords(&mut keywords, "and");
        let but_keywords = get_keywords(&mut keywords, "but");
        let step_keywords = Vec::new();

        let mut dialect = GherkinDialect {
            language,
            name,
            native_name,
            feature_keywords,
            scenario_keywords,
            background_keywords,
            scenario_outline_keywords,
            examples_keywords,
            given_keywords,
            when_keywords,
            then_keywords,
            and_keywords,
            but_keywords,
            step_keywords,
        };
        dialect.init_step_keywords();
        dialect
    }

    pub(crate) fn init_step_keywords(&mut self) {
        let step_keywords = &mut self.step_keywords;
        debug_assert!(step_keywords.is_empty(), "step keywords must only be initialized once");
        step_keywords.extend_from_slice(&self.given_keywords);
        step_keywords.extend_from_slice(&self.when_keywords);
        step_keywords.extend_from_slice(&self.then_keywords);
        step_keywords.extend_from_slice(&self.and_keywords);
        step_keywords.extend_from_slice(&self.but_keywords);
    }

    pub fn get_language(&self) -> &String {
        &self.language
    }

    pub fn get_feature_keywords(&self) -> &Vec<String> {
        &self.feature_keywords
    }

    pub fn get_scenario_keywords(&self) -> &Vec<String> {
        &self.scenario_keywords
    }

    pub fn get_step_keywords(&self) -> &Vec<String> {
        &self.step_keywords
    }

    pub fn get_background_keywords(&self) -> &Vec<String> {
        &self.background_keywords
    }

    pub fn get_scenario_outline_keywords(&self) -> &Vec<String> {
        &self.scenario_outline_keywords
    }

    pub fn get_examples_keywords(&self) -> &Vec<String> {
        &self.examples_keywords
    }

    pub fn get_given_keywords(&self) -> &Vec<String> {
        &self.given_keywords
    }

    pub fn get_when_keywords(&self) -> &Vec<String> {
        &self.when_keywords
    }

    pub fn get_then_keywords(&self) -> &Vec<String> {
        &self.then_keywords
    }

    pub fn get_and_keywords(&self) -> &Vec<String> {
        &self.and_keywords
    }

    pub fn get_but_keywords(&self) -> &Vec<String> {
        &self.but_keywords
    }
}
