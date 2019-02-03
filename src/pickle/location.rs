#[derive(Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub line: u32,
    pub column: u32,
}
