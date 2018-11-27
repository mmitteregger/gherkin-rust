use pickle::*;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PickleCell {
    pub location: PickleLocation,
    pub value: String,
}
