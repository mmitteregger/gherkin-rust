use crate::ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TableRow {
    #[serde(rename = "type")]
    node_type: &'static str,
    pub location: Location,
    pub cells: Vec<TableCell>,
}

impl TableRow {
    pub fn new(location: Location, cells: Vec<TableCell>) -> TableRow {
        TableRow {
            node_type: "TableRow",
            location,
            cells,
        }
    }
}
