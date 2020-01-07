use crate::ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GherkinDocument {
    #[serde(rename = "type")]
    node_type: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<Feature>,
    pub comments: Vec<Comment>,
}

impl GherkinDocument {
    pub fn new(feature: Option<Feature>, comments: Vec<Comment>) -> GherkinDocument {
        GherkinDocument {
            node_type: "GherkinDocument",
            feature,
            comments,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_sync<T: Sync>() {}
    fn assert_send<T: Send>() {}

    #[test]
    fn test_send_sync() {
        assert_send::<GherkinDocument>();
        assert_sync::<GherkinDocument>();
    }
}
