use event::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SourceEvent {
    #[serde(rename = "type")]
    event_type: &'static str,
    uri: String,
    data: String,
    media: Media,
}

impl SourceEvent {
    pub fn new(uri: String, data: String) -> SourceEvent {
        SourceEvent {
            event_type: "source",
            uri,
            data,
            media: Media::new(),
        }
    }

    pub fn get_uri(&self) -> &String {
        &self.uri
    }

    pub fn get_data(&self) -> &String {
        &self.data
    }
}

impl CucumberEvent for SourceEvent {}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    encoding: &'static str,
    media_type: &'static str,
}

impl Media {
    pub fn new() -> Media {
        Media {
            encoding: "utf-8",
            media_type: "text/x.cucumber.gherkin+plain",
        }
    }
}
