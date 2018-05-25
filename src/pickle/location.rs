#[derive(Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PickleLocation {
    line: usize,
    column: usize,
}

impl PickleLocation {
    pub fn new(line: usize, column: usize) -> PickleLocation {
        PickleLocation { line, column }
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_column(&self) -> usize {
        self.column
    }
}
