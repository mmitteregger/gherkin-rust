use ast::*;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Argument {
    DataTable(DataTable),
    DocString(DocString),
}

impl Argument {
    pub fn get_location(&self) -> Location {
        match self {
            Argument::DataTable(data_table) => data_table.get_location(),
            Argument::DocString(doc_string) => doc_string.get_location(),
        }
    }
}

impl From<DataTable> for Argument {
    fn from(data_table: DataTable) -> Self {
        Argument::DataTable(data_table)
    }
}

impl From<DocString> for Argument {
    fn from(doc_string: DocString) -> Self {
        Argument::DocString(doc_string)
    }
}
