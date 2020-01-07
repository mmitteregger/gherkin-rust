use serde::Serialize;

use crate::ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    #[serde(rename = "type")]
    node_type: &'static str,
    pub location: Location,
    pub text: String,
}

impl Comment {
    pub fn new(location: Location, text: String) -> Comment {
        Comment {
            node_type: "Comment",
            location,
            text,
        }
    }
}
