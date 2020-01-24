use std::default::Default;
use std::mem;
use std::string::ToString;

use crate::error::Result;
use crate::line::LineSpan;
use crate::parser::{self, RuleType};
use crate::token::Token;
use crate::Location;

pub struct TokenFormatterBuilder {
    tokens_text_builder: String,
}

impl Default for TokenFormatterBuilder {
    fn default() -> TokenFormatterBuilder {
        TokenFormatterBuilder {
            tokens_text_builder: String::new(),
        }
    }
}

impl parser::Builder for TokenFormatterBuilder {
    type BuilderResult = String;

    fn build(&mut self, token: Token) -> Result<()> {
        let formatted_token = format_token(token);
        self.tokens_text_builder += &formatted_token;
        self.tokens_text_builder += "\n";
        Ok(())
    }

    fn start_rule(&mut self, _rule_type: RuleType) -> Result<()> {
        Ok(())
    }

    fn end_rule(&mut self, _rule_type: RuleType) -> Result<()> {
        Ok(())
    }

    fn get_result(&mut self) -> String {
        mem::replace(&mut self.tokens_text_builder, String::new())
    }

    fn reset(&mut self) {}
}

fn format_token(token: Token) -> String {
    if token.is_eof() {
        return String::from("EOF");
    }

    format!(
        "{}{}:{}/{}/{}",
        format_location(token.location),
        format_option_string(&token.matched_type),
        format_option_string(&token.matched_keyword),
        format_option_string(&token.matched_text),
        format_line_spans(&token.matched_items),
    )
}

fn format_location(location: Option<Location>) -> String {
    match location {
        Some(location) => format!("({}:{})", location.line, location.column),
        None => String::new(),
    }
}

fn format_option_string<S: ToString>(string: &Option<S>) -> String {
    match *string {
        Some(ref s) => s.to_string(),
        None => String::new(),
    }
}

fn format_line_spans(spans: &[LineSpan]) -> String {
    if spans.is_empty() {
        String::new()
    } else {
        spans
            .iter()
            .map(|ref span| format!("{}:{}", span.column, span.text))
            .collect::<Vec<String>>()
            .join(",")
    }
}
