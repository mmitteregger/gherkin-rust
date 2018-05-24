use ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Examples {
    #[serde(rename = "type")]
    node_type: &'static str,
    location: Location,
    tags: Vec<Tag>,
    keyword: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_header: Option<TableRow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_body: Option<Vec<TableRow>>,
}

impl Examples {
    pub fn new(location: Location, tags: Vec<Tag>, keyword: String, name: String,
        description: Option<String>, table_header: Option<TableRow>, table_body: Option<Vec<TableRow>>) -> Examples {
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

    fn get_tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    fn get_keyword(&self) -> &String {
        &self.keyword
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_description(&self) -> &Option<String> {
        &self.description
    }

    fn get_table_header(&self) -> &Option<TableRow> {
        &self.table_header
    }

    fn get_table_body(&self) -> &Option<Vec<TableRow>> {
        &self.table_body
    }
}

impl Node for Examples {
    fn get_location(&self) -> Location {
        self.location
    }
}
