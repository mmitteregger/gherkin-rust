use std::default::Default;

use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SourceEvent {
    #[serde(rename = "type")]
    event_type: &'static str,
    pub uri: String,
    pub data: String,
    media: Media,
}

impl SourceEvent {
    pub fn new(uri: String, data: String) -> SourceEvent {
        SourceEvent {
            event_type: "source",
            uri,
            data,
            media: Media::default(),
        }
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
            media_type: "text/x.cucumber.gherkin+plain",
        }
    }
}
