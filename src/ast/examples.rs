use crate::ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Examples {
    #[serde(rename = "type")]
    node_type: &'static str,
    pub location: Location,
    pub tags: Vec<Tag>,
    pub keyword: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_header: Option<TableRow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_body: Option<Vec<TableRow>>,
}

impl Examples {
    pub fn new(
        location: Location,
        tags: Vec<Tag>,
        keyword: String,
        name: String,
        description: Option<String>,
        table_header: Option<TableRow>,
        table_body: Option<Vec<TableRow>>,
    ) -> Examples {
        Examples {
            node_type: "Examples",
            location,
            tags,
            keyword,
            name,
            description,
            table_header,
            table_body,
        }
    }
}
