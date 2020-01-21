use crate::cuke::{Argument, Location};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Rule<'d> {
    pub location: Location,
    pub keyword: &'d str,
    pub name: &'d str,
    pub description: std::string::String,
    pub background: Option<&'d ast::Background>,
    pub scenarios: Vec<&'d ast::Scenario>,
}
