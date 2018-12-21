use ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TableCell {
    #[serde(rename = "type")]
    node_type: &'static str,
    pub location: Location,
    pub value: String,
}

impl TableCell {
    pub fn new(location: Location, value: String) -> TableCell {
        TableCell {
            node_type: "TableCell",
            location,
            value,
        }
    }
}
