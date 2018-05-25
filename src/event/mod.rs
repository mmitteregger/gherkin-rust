use std::fmt::Debug;

use downcast::Downcast;
use erased_serde::Serialize;

pub use self::attachment_event::AttachmentEvent;
pub use self::gherkin_document_event::GherkinDocumentEvent;
pub use self::pickle_event::PickleEvent;
pub use self::source_event::SourceEvent;
use error::Result;
use pickle::Compiler;
use GherkinDialectProvide;
use Parser;
use TokenMatcher;

pub mod attachment_event;
pub mod gherkin_document_event;
pub mod pickle_event;
pub mod source_event;

pub trait CucumberEvent: Serialize + Downcast + Debug {}

serialize_trait_object!(CucumberEvent);
impl_downcast!(CucumberEvent);

pub fn generate<D: Into<String>, U: AsRef<str>>(
    data: D,
    uri: U,
) -> Result<Vec<Box<CucumberEvent>>> {
    generate_with_matcher(data, uri, TokenMatcher::default())
}

pub fn generate_with_language<D: Into<String>, U: AsRef<str>, L: Into<String>>(
    data: D,
    uri: U,
    language: L,
) -> Result<Vec<Box<CucumberEvent>>> {
    let token_matcher = TokenMatcher::with_default_dialect_name(language);
    generate_with_matcher(data, uri, token_matcher)
}

fn generate_with_matcher<D: Into<String>, U: AsRef<str>, DP: GherkinDialectProvide>(
    data: D,
    uri: U,
    mut token_matcher: TokenMatcher<DP>,
) -> Result<Vec<Box<CucumberEvent>>> {
    let data = data.into();
    let uri = uri.as_ref();

    let mut parser = Parser::default();
    let mut compiler = Compiler::default();

    let gherkin_document = parser.parse_str_with_matcher(&data, &mut token_matcher)?;
    let pickles = compiler.compile(&gherkin_document);

    let mut events: Vec<Box<CucumberEvent>> = Vec::with_capacity(2 + pickles.len());
    events.push(Box::new(SourceEvent::new(data, uri.to_owned())));
    events.push(Box::new(GherkinDocumentEvent::new(
        uri.to_owned(),
        gherkin_document,
    )));
    for pickle in pickles {
        events.push(Box::new(PickleEvent::new(uri.to_owned(), pickle)));
    }

    Ok(events)
}
