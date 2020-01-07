use serde::Serialize;

pub use self::attachment_event::AttachmentEvent;
pub use self::gherkin_document_event::GherkinDocumentEvent;
pub use self::pickle_event::PickleEvent;
pub use self::source_event::SourceEvent;
use crate::error::Result;
use crate::parser::GherkinDialectProvide;
use crate::parser::ParserOptions;
use crate::pickle::Pickle;
use crate::cuke::Compiler;
use crate::token_matcher::TokenMatcher;

pub mod attachment_event;
pub mod gherkin_document_event;
pub mod pickle_event;
pub mod source_event;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum CucumberEvent {
    Attachment(AttachmentEvent),
    GherkinDocument(GherkinDocumentEvent),
    Pickle(PickleEvent),
    Source(SourceEvent),
}


impl From<AttachmentEvent> for CucumberEvent {
    fn from(attachment_event: AttachmentEvent) -> Self {
        CucumberEvent::Attachment(attachment_event)
    }
}

impl From<GherkinDocumentEvent> for CucumberEvent {
    fn from(gherkin_document_event: GherkinDocumentEvent) -> Self {
        CucumberEvent::GherkinDocument(gherkin_document_event)
    }
}

impl From<PickleEvent> for CucumberEvent {
    fn from(pickle_event: PickleEvent) -> Self {
        CucumberEvent::Pickle(pickle_event)
    }
}

impl From<SourceEvent> for CucumberEvent {
    fn from(source_event: SourceEvent) -> Self {
        CucumberEvent::Source(source_event)
    }
}

pub fn generate<D, U>(data: D, uri: U) -> Result<Vec<CucumberEvent>>
where
    D: Into<String>,
    U: AsRef<str>,
{
    generate_with_matcher(data, uri, TokenMatcher::default())
}

pub fn generate_with_language<D, U, L>(
    data: D,
    uri: U,
    language: L,
) -> Result<Vec<CucumberEvent>>
where
    D: Into<String>,
    U: AsRef<str>,
    L: Into<String>,
{
    let token_matcher = TokenMatcher::with_default_dialect_name(language);
    generate_with_matcher(data, uri, token_matcher)
}

fn generate_with_matcher<D, U, DP>(
    data: D,
    uri: U,
    token_matcher: TokenMatcher<DP>,
) -> Result<Vec<CucumberEvent>>
where
    D: Into<String>,
    U: AsRef<str>,
    DP: GherkinDialectProvide + 'static,
{
    let data = data.into();
    let uri = uri.as_ref();

    let mut parser = ParserOptions::new().token_matcher(token_matcher).create();
    let mut compiler = Compiler::default();

    let gherkin_document = parser.parse_str(&data)?;
    let pickles: Vec<Pickle> = compiler.compile(&gherkin_document)
        .into_iter()
        .map(Pickle::from)
        .collect();

    let mut events: Vec<CucumberEvent> = Vec::with_capacity(2 + pickles.len());
    events.push(CucumberEvent::from(SourceEvent::new(data, uri.to_owned())));
    events.push(CucumberEvent::from(GherkinDocumentEvent::new(
        uri.to_owned(),
        gherkin_document,
    )));
    for pickle in pickles {
        events.push(CucumberEvent::from(PickleEvent::new(uri.to_owned(), pickle)));
    }

    Ok(events)
}
