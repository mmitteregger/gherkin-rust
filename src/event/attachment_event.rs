use std::default::Default;

use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentEvent {
    #[serde(rename = "type")]
    event_type: &'static str,
    pub source: SourceRef,
    pub data: String,
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

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SourceRef {
    pub uri: String,
    pub start: Location,
}

impl SourceRef {
    pub fn new(uri: String, start: Location) -> SourceRef {
        SourceRef { uri, start }
    }
}

#[derive(Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

impl Location {
    pub fn new(line: u32, column: u32) -> Location {
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
