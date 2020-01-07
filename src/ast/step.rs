use crate::ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    #[serde(rename = "type")]
    node_type: &'static str,
    pub location: Location,
    pub keyword: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argument: Option<Argument>,
}

impl Step {
    pub fn new(
        location: Location,
        keyword: String,
        text: String,
        argument: Option<Argument>,
    ) -> Step {
        Step {
            node_type: "Step",
            location,
            keyword,
            text,
            argument,
        }
    }
}
