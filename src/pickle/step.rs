use pickle::*;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PickleStep {
    pub text: String,
    pub arguments: Vec<Argument>,
    pub locations: Vec<PickleLocation>,
}
