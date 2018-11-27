use pickle::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PickleTag {
    pub location: PickleLocation,
    pub name: String,
}
