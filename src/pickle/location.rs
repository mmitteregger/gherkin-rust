#[derive(Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PickleLocation {
    line: u32,
    column: u32,
}

impl PickleLocation {
    pub fn new(line: u32, column: u32) -> PickleLocation {
        PickleLocation { line, column }
    }

    pub fn get_line(&self) -> u32 {
        self.line
    }

    pub fn get_column(&self) -> u32 {
        self.column
    }
}
