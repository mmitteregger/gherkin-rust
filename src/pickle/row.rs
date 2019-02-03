use pickle::Cell;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub cells: Vec<Cell>,
}
