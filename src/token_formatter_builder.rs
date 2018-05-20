use std::mem;
use std::rc::Rc;
use std::cell::RefCell;
use std::string::ToString;
use error::Result;
use ast::Location;
use token::Token;
use gherkin_line_span::GherkinLineSpan;
use parser::{self, RuleType};

pub struct TokenFormatterBuilder {
    tokens_text_builder: String,
}

impl TokenFormatterBuilder {
    pub fn new() -> TokenFormatterBuilder {
        TokenFormatterBuilder {
            tokens_text_builder: String::new(),
        }
    }
}

impl parser::Builder for TokenFormatterBuilder {
    type BuilderResult = String;

    fn build(&mut self, token: Rc<RefCell<Token>>) -> Result<()> {
        let formatted_token = format_token(&*token.borrow());
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

    fn reset(&mut self) {
    }
}

fn format_token(token: &Token) -> String {
    if token.is_eof() {
        return String::from("EOF");
    }

    format!("{}{}:{}/{}/{}",
        format_location(&token.location),
        format_option_string(&token.matched_type),
        format_option_string(&token.matched_keyword),
        format_option_string(&token.matched_text),
        format_gherkin_line_spans(&token.matched_items),
    )
}

fn format_location(o: &Option<Location>) -> String {
    match o {
        &Some(ref location) => format!("({}:{})", location.get_line(), location.get_column()),
        &None => String::new(),
    }
}

fn format_option_string<S: ToString>(o: &Option<S>) -> String {
    match o {
        &Some(ref s) => s.to_string(),
        &None => String::new(),
    }
}

fn format_gherkin_line_spans(spans: &Vec<GherkinLineSpan>) -> String {
    if spans.is_empty() {
        String::new()
    } else {
        spans.iter()
            .map(|ref span| format!("{}:{}", span.get_column(), span.get_text()))
            .collect::<Vec<String>>()
            .join(",")
    }
}
