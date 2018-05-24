use ast::Location;
use error::{ErrorKind, Result};
use gherkin_dialect::GherkinDialect;
use parser::GherkinDialectProvide;
use serde_json;
use std::collections::HashMap;
use std::default::Default;
use std::sync::Arc;

static GHERKIN_LANGUAGES: &[u8] = include_bytes!("../gherkin-languages.json");

lazy_static! {
    static ref DIALECTS: HashMap<String, Arc<GherkinDialect>> = parse_built_int_gherkin_dialects();
}

fn parse_built_int_gherkin_dialects() -> HashMap<String, Arc<GherkinDialect>> {
    let dialects: HashMap<String, GherkinDialect> =
        serde_json::from_slice(GHERKIN_LANGUAGES).unwrap();
    let mut arc_dialects = HashMap::with_capacity(dialects.len());

    for (language, mut dialect) in dialects {
        dialect.language = language.clone();
        dialect.init_step_keywords();
        arc_dialects.insert(language, Arc::new(dialect));
    }

    arc_dialects
}

pub struct BuiltInGherkinDialectProvider {
    default_dialect_name: String,
}

impl Default for BuiltInGherkinDialectProvider {
    fn default() -> BuiltInGherkinDialectProvider {
        BuiltInGherkinDialectProvider {
            default_dialect_name: String::from("en"),
        }
    }
}

impl BuiltInGherkinDialectProvider {
    pub fn with_default_dialect_name(default_dialect_name: String) -> BuiltInGherkinDialectProvider {
        BuiltInGherkinDialectProvider {
            default_dialect_name,
        }
    }
}

impl GherkinDialectProvide for BuiltInGherkinDialectProvider {
    fn get_default_dialect(&self) -> Result<Arc<GherkinDialect>> {
        self.get_dialect(&self.default_dialect_name, None)
    }

    fn get_dialect(&self, language: &str, location: Option<Location>)
            -> Result<Arc<GherkinDialect>> {
        let dialect = DIALECTS.get(language);
        let language = language.to_owned();

        match dialect {
            Some(dialect) => Ok(dialect.clone()),
            None => {
                Err(ErrorKind::NoSuchLanguage {
                    location,
                    language,
                }.into())
            },
        }
    }

    fn get_languages(&self) -> Vec<&String> {
        let mut languages: Vec<&String> = DIALECTS.keys().collect();
        languages.sort_unstable();
        languages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provides_emoji_dialect() {
        let em_dialect = get_dialect("em");
        let scenario_keywords = em_dialect.get_scenario_keywords();
        let first_emoji_keyword = scenario_keywords.get(0).unwrap();
        assert_eq!(first_emoji_keyword.chars().count(), 1,
            "expected exactly 1 char for first emoji scenario keyword: {}", first_emoji_keyword);
    }

    fn get_dialect(language: &str) -> Arc<GherkinDialect> {
        let dialect_provider = BuiltInGherkinDialectProvider::default();
        let dialect = dialect_provider.get_dialect(language, &None).unwrap();
        dialect
    }
}
