use ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TableRow {
    #[serde(rename = "type")]
    node_type: &'static str,
    location: Location,
    cells: Vec<TableCell>,
}

impl TableRow {
    pub fn new(location: Location, cells: Vec<TableCell>) -> TableRow {
        TableRow {
            node_type: "TableRow",
            location,
            cells,
        }
    }

    pub fn get_cells(&self) -> &Vec<TableCell> {
        &self.cells
    }
}

impl Node for TableRow {
    fn get_location(&self) -> Location {
        self.location
    }
}
