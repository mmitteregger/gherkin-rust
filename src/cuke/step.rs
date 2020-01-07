use crate::cuke::{Argument, Location};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Step<'d> {
    pub keyword: &'d str,
    pub text: Cow<'d, str>,
    pub argument: Option<Argument<'d>>,
    pub locations: Vec<Location>,
}
