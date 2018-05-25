use std::default::Default;

use event::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentEvent {
    #[serde(rename = "type")]
    event_type: &'static str,
    source: SourceRef,
    data: String,
    media: Media,
}

impl AttachmentEvent {
    pub fn new(source: SourceRef, data: String) -> AttachmentEvent {
        AttachmentEvent {
            event_type: "attachment",
            source,
            data,
            media: Media::default(),
        }
    }
}

impl CucumberEvent for AttachmentEvent {}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SourceRef {
    uri: String,
    start: Location,
}

impl SourceRef {
    pub fn new(uri: String, start: Location) -> SourceRef {
        SourceRef { uri, start }
    }
}

#[derive(Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    line: usize,
    column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Location {
        Location { line, column }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    encoding: &'static str,
    #[serde(rename = "type")]
    media_type: &'static str,
}

impl Default for Media {
    fn default() -> Media {
        Media {
            encoding: "utf-8",
            media_type: "text/x.cucumber.stacktrace+plain",
        }
    }
}
