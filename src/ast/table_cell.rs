use ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TableCell {
    #[serde(rename = "type")]
    node_type: &'static str,
    location: Location,
    value: String,
}

impl TableCell {
    pub fn new(location: Location, value: String) -> TableCell {
        TableCell {
            node_type: "TableCell",
            location,
            value,
        }
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}

impl Node for TableCell {
    fn get_location(&self) -> Location {
        self.location
    }
}
