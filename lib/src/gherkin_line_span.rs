#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct GherkinLineSpan {
    pub column: u32,
    pub text: String,
}
