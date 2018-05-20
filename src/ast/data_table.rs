use ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataTable {
    #[serde(rename = "type")]
    node_type: &'static str,
    location: Location,
    rows: Vec<TableRow>,
}

impl DataTable {
    pub fn new(rows: Vec<TableRow>) -> DataTable {
        DataTable {
            node_type: "DataTable",
            location: rows.get(0).expect("no rows for data table").get_location(),
            rows,
        }
    }

    pub fn get_rows(&self) -> &Vec<TableRow> {
        &self.rows
    }
}

impl Node for DataTable {
    fn get_location(&self) -> Location {
        self.location
    }
}
