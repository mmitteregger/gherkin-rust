use event::*;
use ast::GherkinDocument;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GherkinDocumentEvent {
    #[serde(rename = "type")]
    event_type: &'static str,
    uri: String,
    document: GherkinDocument,
}

impl GherkinDocumentEvent {
    pub fn new(uri: String, document: GherkinDocument) -> GherkinDocumentEvent {
        GherkinDocumentEvent {
            event_type: "gherkin-document",
            uri,
            document,
        }
    }
}

impl CucumberEvent for GherkinDocumentEvent {}
