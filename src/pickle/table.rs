use crate::pickle::{Row, Location};
use crate::cuke;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub rows: Vec<Row>,
}

impl<'d> From<cuke::Table<'d>> for Table {
    fn from(cuke_table: cuke::Table<'d>) -> Self {
        Table {
            rows: cuke_table.rows.into_iter().map(Row::from).collect(),
        }
    }
}

impl Table {
    pub fn get_location(&self) -> &Location {
        &self.rows[0].cells[0].location
    }
}
