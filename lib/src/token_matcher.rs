use std::default::Default;
use std::sync::Arc;

use regex::Regex;

use lazy_static::lazy_static;

use crate::constant;
use crate::dialect::Dialect;
use crate::dialect_provider::BuiltInDialectProvider;
use crate::error::Result;
use crate::line::LineSpan;
use crate::location::Location;
use crate::parser::DialectProvider;
use crate::parser::{TokenMatch, TokenType};
use crate::token::Token;

lazy_static! {
    static ref LANGUAGE_PATTERN: Regex =
        Regex::new(r"^\s*#\s*language\s*:\s*([a-zA-Z\-_]+)\s*$").unwrap();
}

pub struct TokenMatcher<DP: DialectProvider> {
    dialect_provider: DP,
    current_dialect: Arc<Dialect>,
    active_doc_string_separator: Option<String>,
    indent_to_remove: u32,
}

impl Default for TokenMatcher<BuiltInDialectProvider> {
    fn default() -> TokenMatcher<BuiltInDialectProvider> {
        TokenMatcher::with_dialect_provider(BuiltInDialectProvider::default())
    }
}

impl TokenMatcher<BuiltInDialectProvider> {
    pub fn with_default_dialect_name<S>(default: S) -> TokenMatcher<BuiltInDialectProvider>
    where
        S: Into<String>,
    {
        let dialect_provider = BuiltInDialectProvider::with_default_dialect_name(default);
        TokenMatcher::with_dialect_provider(dialect_provider)
    }
}

impl<DP: DialectProvider> TokenMatcher<DP> {
    pub fn with_dialect_provider(dialect_provider: DP) -> TokenMatcher<DP> {
        let default_dialect = dialect_provider
            .get_default_dialect()
            .expect("get default dialect");

        TokenMatcher {
            dialect_provider,
            active_doc_string_separator: None,
            indent_to_remove: 0,
            current_dialect: default_dialect,
        }
    }

    fn set_token_matched(
        &self,
        token: &mut Token,
        matched_type: TokenType,
        text: Option<String>,
        keyword: Option<String>,
        indent: Option<u32>,
        items: Vec<LineSpan>,
    ) {
        token.matched_type = Some(matched_type);
        token.matched_keyword = keyword;
        token.matched_text = text;
        token.matched_items = items;
        token.matched_dialect = Some(self.current_dialect.clone());
        token.matched_indent = indent.or_else(|| match token.line {
            Some(ref line) => Some(line.indent()),
            None => Some(0),
        });
        let token_location = {
            let location = token.location.as_ref().expect("token location");
            let line = location.line;
            let column = token.matched_indent.unwrap() + 1;
            Location::new(line, column)
        };
        token.location = Some(token_location);
    }

    fn unescape_doc_string(&self, text: &str) -> String {
        match self.active_doc_string_separator.as_deref() {
            Some(constant::DOCSTRING_SEPARATOR) => {
                text.replace(r#"\"\"\""#, constant::DOCSTRING_SEPARATOR)
            }
            Some(constant::DOCSTRING_ALTERNATIVE_SEPARATOR) => {
                text.replace(r#"\`\`\`"#, constant::DOCSTRING_ALTERNATIVE_SEPARATOR)
            }
            Some(docstring_separator) => {
                panic!(
                    "cannot escape unexpected docstring separator: {}",
                    docstring_separator
                );
            }
            None => text.to_owned(),
        }
    }

    fn match_title_line(
        &self,
        token: &mut Token,
        token_type: TokenType,
        keywords: &[String],
    ) -> bool {
        for keyword in keywords {
            if token.unwrap_line().starts_with_title_keyword(keyword) {
                let keyword_chars_count = keyword.chars().count();
                let separator_chars_count = constant::TITLE_KEYWORD_SEPARATOR.chars().count();
                let title = {
                    let line = token.unwrap_line();
                    line.get_rest_trimmed(keyword_chars_count + separator_chars_count)
                        .to_owned()
                };
                self.set_token_matched(
                    token,
                    token_type,
                    Some(title),
                    Some(keyword.to_owned()),
                    None,
                    Vec::new(),
                );
                return true;
            }
        }

        false
    }

    fn match_doc_string_separator(
        &mut self,
        token: &mut Token,
        separator: &str,
        is_open: bool,
    ) -> bool {
        if token.unwrap_line().starts_with(&separator) {
            let content_type = if is_open {
                let line = token.unwrap_line();
                let separator_chars_count = separator.chars().count();
                self.active_doc_string_separator = Some(separator.to_owned());
                self.indent_to_remove = line.indent();
                Some(line.get_rest_trimmed(separator_chars_count).to_owned())
            } else {
                self.active_doc_string_separator = None;
                self.indent_to_remove = 0;
                None
            };

            self.set_token_matched(
                token,
                TokenType::DocStringSeparator,
                content_type,
                Some(separator.to_owned()),
                None,
                Vec::new(),
            );
            return true;
        }

        false
    }
}

impl<DP: DialectProvider> TokenMatch for TokenMatcher<DP> {
    fn match_eof(&mut self, token: &mut Token) -> Result<bool> {
        if token.is_eof() {
            self.set_token_matched(token, TokenType::Eof, None, None, None, Vec::new());
            return Ok(true);
        }
        Ok(false)
    }

    fn match_empty(&mut self, token: &mut Token) -> Result<bool> {
        if token.unwrap_line().is_empty() {
            self.set_token_matched(token, TokenType::Empty, None, None, None, Vec::new());
            return Ok(true);
        }
        Ok(false)
    }

    fn match_comment(&mut self, token: &mut Token) -> Result<bool> {
        if token.unwrap_line().starts_with(constant::COMMENT_PREFIX) {
            // take the entire line
            let text = token.unwrap_line().get_text(0).to_owned();
            self.set_token_matched(
                token,
                TokenType::Comment,
                Some(text),
                None,
                Some(0),
                Vec::new(),
            );
            return Ok(true);
        }
        Ok(false)
    }

    fn match_tag_line(&mut self, token: &mut Token) -> Result<bool> {
        if token.unwrap_line().starts_with(constant::TAG_PREFIX) {
            let tags = token.unwrap_line().get_tags()?;
            self.set_token_matched(token, TokenType::TagLine, None, None, None, tags);
            return Ok(true);
        }
        Ok(false)
    }

    fn match_feature_line(&mut self, token: &mut Token) -> Result<bool> {
        let keywords = self.current_dialect.get_feature_keywords();
        let is_match = self.match_title_line(token, TokenType::FeatureLine, keywords);
        Ok(is_match)
    }

    fn match_rule_line(&mut self, token: &mut Token) -> Result<bool> {
        let keywords = self.current_dialect.get_rule_keywords();
        let is_match = self.match_title_line(token, TokenType::RuleLine, keywords);
        Ok(is_match)
    }

    fn match_background_line(&mut self, token: &mut Token) -> Result<bool> {
        let keywords = self.current_dialect.get_background_keywords();
        let is_match = self.match_title_line(token, TokenType::BackgroundLine, keywords);
        Ok(is_match)
    }

    fn match_scenario_line(&mut self, token: &mut Token) -> Result<bool> {
        let scenario_keywords = self.current_dialect.get_scenario_keywords();
        let scenario_outline_keywords = self.current_dialect.get_scenario_outline_keywords();
        let is_match = self.match_title_line(token, TokenType::ScenarioLine, scenario_keywords)
            || self.match_title_line(token, TokenType::ScenarioLine, scenario_outline_keywords);
        Ok(is_match)
    }

    fn match_examples_line(&mut self, token: &mut Token) -> Result<bool> {
        let keywords = self.current_dialect.get_examples_keywords();
        let is_match = self.match_title_line(token, TokenType::ExamplesLine, keywords);
        Ok(is_match)
    }

    fn match_step_line(&mut self, token: &mut Token) -> Result<bool> {
        let keywords = self.current_dialect.get_step_keywords();

        for keyword in keywords {
            if token.unwrap_line().starts_with(keyword) {
                let keyword_chars_count = keyword.chars().count();
                let step_text = token
                    .unwrap_line()
                    .get_rest_trimmed(keyword_chars_count)
                    .to_owned();
                self.set_token_matched(
                    token,
                    TokenType::StepLine,
                    Some(step_text),
                    Some(keyword.to_owned()),
                    None,
                    Vec::new(),
                );
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn match_doc_string_separator(&mut self, token: &mut Token) -> Result<bool> {
        let is_match = match self.active_doc_string_separator.to_owned() {
            Some(ref separator) => {
                // close
                self.match_doc_string_separator(token, separator, false)
            }
            None => {
                // open
                self.match_doc_string_separator(token, constant::DOCSTRING_SEPARATOR, true)
                    || self.match_doc_string_separator(
                        token,
                        constant::DOCSTRING_ALTERNATIVE_SEPARATOR,
                        true,
                    )
            }
        };
        Ok(is_match)
    }

    fn match_table_row(&mut self, token: &mut Token) -> Result<bool> {
        if token
            .unwrap_line()
            .starts_with(constant::TABLE_CELL_SEPARATOR)
        {
            let table_cells = token.unwrap_line().get_table_cells();
            self.set_token_matched(token, TokenType::TableRow, None, None, None, table_cells);
            return Ok(true);
        }
        Ok(false)
    }

    fn match_language(&mut self, token: &mut Token) -> Result<bool> {
        let language = {
            let line_text = token.unwrap_line().get_text(0);
            let captures = LANGUAGE_PATTERN.captures(line_text);

            match captures {
                Some(captures) => captures.get(1).map(|mat| mat.as_str().to_owned()),
                None => None,
            }
        };

        if let Some(language) = language {
            self.set_token_matched(
                token,
                TokenType::Language,
                Some(language),
                None,
                None,
                Vec::new(),
            );

            let location = token.location.expect("token location");
            let dialect_language = token.matched_text.as_ref().unwrap();
            self.current_dialect = self
                .dialect_provider
                .get_dialect(dialect_language, location)?;
            return Ok(true);
        }

        Ok(false)
    }

    fn match_other(&mut self, token: &mut Token) -> Result<bool> {
        // take the entire line, except removing DocString indents
        let text = {
            let line = token.unwrap_line();
            let line_text = line.get_text(self.indent_to_remove as isize);
            self.unescape_doc_string(line_text)
        };
        self.set_token_matched(
            token,
            TokenType::Other,
            Some(text),
            None,
            Some(0),
            Vec::new(),
        );
        Ok(true)
    }

    fn reset(&mut self) {
        self.active_doc_string_separator = None;
        self.indent_to_remove = 0;
        self.current_dialect = self
            .dialect_provider
            .get_default_dialect()
            .expect("get default dialect");
    }
}
