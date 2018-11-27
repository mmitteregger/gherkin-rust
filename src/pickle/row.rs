use pickle::*;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PickleRow {
    pub cells: Vec<PickleCell>,
}
