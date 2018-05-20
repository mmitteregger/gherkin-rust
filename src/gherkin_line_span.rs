#[derive(PartialEq, Eq, Hash, Debug)]
pub struct GherkinLineSpan {
    column: usize,
    text: String,
}

impl GherkinLineSpan {
    pub fn new(column: usize, text: String) -> GherkinLineSpan {
        GherkinLineSpan {
            column,
            text,
        }
    }

    /// One-based line position.
    pub fn get_column(&self) -> usize {
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
