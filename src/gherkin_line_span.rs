#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct GherkinLineSpan {
    column: u32,
    text: String,
}

impl GherkinLineSpan {
    pub fn new(column: u32, text: String) -> GherkinLineSpan {
        GherkinLineSpan { column, text }
    }

    /// One-based line position.
    pub fn get_column(&self) -> u32 {
        self.column
    }

    /// Text part of the line.
    pub fn get_text(&self) -> &String {
        &self.text
    }

    /// Text part of the line.
    ///
    /// Moves the text out of this struct by consuming it.
    pub fn take_text(self) -> String {
        self.text
    }
}
