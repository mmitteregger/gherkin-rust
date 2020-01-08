//use serde::Serialize;
//
//use crate::ast::GherkinDocument;
//
//#[derive(Serialize, Debug)]
//#[serde(rename_all = "camelCase")]
//pub struct GherkinDocumentEvent {
//    #[serde(rename = "type")]
//    event_type: &'static str,
//    pub uri: String,
//    pub document: GherkinDocument,
//}
//
//impl GherkinDocumentEvent {
//    pub fn new(uri: String, document: GherkinDocument) -> GherkinDocumentEvent {
//        GherkinDocumentEvent {
//            event_type: "gherkin-document",
//            uri,
//            document,
//        }
//    }
//}
