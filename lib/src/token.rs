use std::sync::Arc;

use crate::dialect::Dialect;
use crate::line::{Line, LineSpan};
use crate::parser::TokenType;
use crate::Location;

#[derive(Debug, Clone)]
pub struct Token {
    pub line: Option<Line>,
    pub matched_type: Option<TokenType>,
    pub matched_keyword: Option<String>,
    pub matched_text: Option<String>,
    pub matched_items: Vec<LineSpan>,
    pub matched_indent: Option<u32>,
    pub matched_dialect: Option<Arc<Dialect>>,
    pub location: Option<Location>,
}

impl Token {
    pub fn new(line: Option<Line>, location: Option<Location>) -> Token {
        Token {
            line,
            location,
            matched_type: None,
            matched_keyword: None,
            matched_text: None,
            matched_items: Vec::new(),
            matched_indent: None,
            matched_dialect: None,
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
            Some(ref line) => line.get_text(-1),
            None => "EOF",
        }
    }

    pub fn unwrap_line(&self) -> &Line {
        &self.line.as_ref().expect("token line")
    }
}
