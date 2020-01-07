use serde::Serialize;

use crate::ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataTable {
    #[serde(rename = "type")]
    node_type: &'static str,
    pub location: Location,
    pub rows: Vec<TableRow>,
}

impl DataTable {
    pub fn new(rows: Vec<TableRow>) -> DataTable {
        DataTable {
            node_type: "DataTable",
            location: rows.get(0).expect("no rows for data table").location,
            rows,
        }
    }
}
