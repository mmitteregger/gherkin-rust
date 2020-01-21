use std::sync::Arc;

use crate::Location;
use crate::gherkin_dialect::GherkinDialect;
use crate::gherkin_line::GherkinLine;
use crate::gherkin_line_span::GherkinLineSpan;
use crate::parser::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub line: Option<GherkinLine>,
    pub matched_type: Option<TokenType>,
    pub matched_keyword: Option<String>,
    pub matched_text: Option<String>,
    pub matched_items: Vec<GherkinLineSpan>,
    pub matched_indent: Option<u32>,
    pub matched_gherkin_dialect: Option<Arc<GherkinDialect>>,
    pub location: Option<Location>,
}

impl Token {
    pub fn new(line: Option<GherkinLine>, location: Option<Location>) -> Token {
        Token {
            line,
            location,
            matched_type: None,
            matched_keyword: None,
            matched_text: None,
            matched_items: Vec::new(),
            matched_indent: None,
            matched_gherkin_dialect: None,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.line.is_none()
    }

    pub fn detach(&self) {
        if let Some(ref line) = self.line {
            line.detach();
        }
    }

    pub fn get_token_value(&self) -> &str {
        match self.line {
            Some(ref line) => line.get_line_text(-1),
            None => "EOF",
        }
    }

    pub fn unwrap_line(&self) -> &GherkinLine {
        &self.line.as_ref().expect("token line")
    }
}
