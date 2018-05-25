use ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GherkinDocument {
    #[serde(rename = "type")]
    node_type: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    feature: Option<Feature>,
    comments: Vec<Comment>,
}

impl GherkinDocument {
    pub fn new(feature: Option<Feature>, comments: Vec<Comment>) -> GherkinDocument {
        GherkinDocument {
            node_type: "GherkinDocument",
            feature,
            comments,
        }
    }

    pub fn get_feature(&self) -> Option<&Feature> {
        self.feature.as_ref()
    }

    pub fn get_comments(&self) -> &Vec<Comment> {
        &self.comments
    }
}

impl Node for GherkinDocument {
    fn get_location(&self) -> Location {
        Location::new(0, 0)
    }
}
