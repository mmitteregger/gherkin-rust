use ast_builder::AstBuilder;
use error::Result;
use event::*;
use gherkin_dialect_provider::BuiltInGherkinDialectProvider;
use parser::Parser;
use pickle::Compiler;
use token_matcher::TokenMatcher;

pub struct GherkinEvents {
    parser: Parser<AstBuilder>,
    matcher: TokenMatcher<BuiltInGherkinDialectProvider>,
    compiler: Compiler,

    print_source: bool,
    print_ast: bool,
    print_pickles: bool,
}

impl GherkinEvents {
    pub fn new(print_source: bool, print_ast: bool, print_pickles: bool) -> GherkinEvents {
        GherkinEvents {
            parser: Parser::default(),
            matcher: TokenMatcher::default(),
            compiler: Compiler::default(),
            print_source,
            print_ast,
            print_pickles,
        }
    }

    pub fn iter_source_event(&mut self, source_event: SourceEvent) -> Result<GherkinEventsIter> {
        let mut cucumber_events: Vec<Box<CucumberEvent>> = Vec::new();

//        try {
        let gherkin_document =
            self.parser.parse_str_with_token_matcher(source_event.get_data(), &mut self.matcher)?;

        let source_event_uri = source_event.get_uri().to_owned();

        if self.print_source {
            cucumber_events.push(Box::new(source_event));
        }

        let pickles = if self.print_pickles {
            self.compiler.compile(&gherkin_document)
        } else {
            Vec::new()
        };

        if self.print_ast {
            let uri = source_event_uri.clone();
            cucumber_events.push(Box::new(GherkinDocumentEvent::new(uri, gherkin_document)));
        }

        for pickle in pickles {
            let uri = source_event_uri.clone();
            cucumber_events.push(Box::new(PickleEvent::new(uri, pickle)));
        }

//        } catch (ParserException.CompositeParserException e) {
//            for (ParserException error : e.errors) {
//                addErrorAttachment(cucumberEvents, error, sourceEvent.uri);
//            }
//        } catch (ParserException e) {
//            addErrorAttachment(cucumberEvents, e, sourceEvent.uri);
//        }

        let gherkin_events_iter = GherkinEventsIter {
            cucumber_events_iter: cucumber_events.into_iter(),
        };
        Ok(gherkin_events_iter)
    }

//    private void addErrorAttachment(List<CucumberEvent> cucumberEvents, ParserException e, String uri) {
//        cucumberEvents.add(new AttachmentEvent(
//                new AttachmentEvent.SourceRef(
//                        uri,
//                        new AttachmentEvent.Location(
//                                e.location.getLine(),
//                                e.location.getColumn()
//                        )
//                ),
//                e.getMessage()
//        ));
//
//    }
}

pub struct GherkinEventsIter {
    cucumber_events_iter: ::std::vec::IntoIter<Box<CucumberEvent>>,
}

impl Iterator for GherkinEventsIter {
    type Item = Box<CucumberEvent>;

    fn next(&mut self) -> Option<Box<CucumberEvent>> {
        self.cucumber_events_iter.next()
    }
}
