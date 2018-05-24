#[derive(Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    line: usize,
    column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Location {
        Location {
            line,
            column,
        }
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_column(&self) -> usize {
        self.column
    }
}