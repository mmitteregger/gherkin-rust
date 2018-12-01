use pickle::*;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PickleStep {
    pub text: String,
    pub arguments: Vec<PickleArgument>,
    pub locations: Vec<PickleLocation>,
}
