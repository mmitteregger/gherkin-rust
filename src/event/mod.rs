use std::fmt::Debug;

use erased_serde::Serialize;

use Parser;
use TokenMatcher;
use GherkinDialectProvide;

pub use self::attachment_event::*;
pub use self::gherkin_document_event::*;
pub use self::pickle_event::*;
pub use self::source_event::*;

mod attachment_event;
mod gherkin_document_event;
mod pickle_event;
mod source_event;

pub trait CucumberEvent: Serialize + Debug {}

serialize_trait_object!(CucumberEvent);

pub fn generate_with_defaults(data: String, uri: String) -> Vec<Box<CucumberEvent>> {
    generate_with_token_matcher(data, uri, TokenMatcher::default())
}

pub fn generate_with_language(data: String, uri: String, language: String) -> Vec<Box<CucumberEvent>> {
    let token_matcher = TokenMatcher::with_default_dialect_name(language);
    generate_with_token_matcher(data, uri, token_matcher)
}

#[allow(unused)] // until the function is implemented
fn generate_with_token_matcher<DP: GherkinDialectProvide>(data: String, uri: String,
        token_matcher: TokenMatcher<DP>) -> Vec<Box<CucumberEvent>> {

    let parser = Parser::default();
    unimplemented!();
//    Parser<GherkinDocument> parser = new Parser<>(new AstBuilder());
//    Compiler compiler = new Compiler();
//
//    List<CucumberEvent> events = new ArrayList<>();
//    events.add(new SourceEvent(data, uri));
//    GherkinDocument document = parser.parse(data, tokenMatcher);
//    events.add(new GherkinDocumentEvent(uri, document));
//    List<Pickle> pickles = compiler.compile(document);
//    for (Pickle pickle : pickles) {
//        events.add(new PickleEvent(uri, pickle));
//    }
//    return events;
}
