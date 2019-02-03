use pickle::{Argument, Location};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    pub text: String,
    pub arguments: Vec<Argument>,
    pub locations: Vec<Location>,
}
