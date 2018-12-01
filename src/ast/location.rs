use std::fmt;

#[derive(Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    line: u32,
    column: u32,
}

impl Location {
    pub fn new(line: u32, column: u32) -> Location {
        Location { line, column }
    }

    pub fn get_line(self) -> u32 {
        self.line
    }

    pub fn get_column(self) -> u32 {
        self.column
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "({}:{})", self.line, self.column)
    }
}
