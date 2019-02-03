use ast::Location;
use ast_builder::AstBuilder;
use error::Error;
use event::*;
use parser::Parser;
use pickle::Pickle;
use cuke::Compiler;

pub struct GherkinEvents {
    parser: Parser<AstBuilder>,
    compiler: Compiler,

    print_source: bool,
    print_ast: bool,
    print_pickles: bool,
}

impl GherkinEvents {
    pub fn new(print_source: bool, print_ast: bool, print_pickles: bool) -> GherkinEvents {
        GherkinEvents {
            parser: Parser::default(),
            compiler: Compiler::default(),
            print_source,
            print_ast,
            print_pickles,
        }
    }

    pub fn iter_source_event(&mut self, source_event: SourceEvent) -> GherkinEventsIter {
        let mut cucumber_events: Vec<CucumberEvent> = Vec::new();

        let uri = &source_event.uri.to_owned();
        let gherkin_document = match self.parser.parse_str(&source_event.data) {
            Ok(gherkin_document) => gherkin_document,
            Err(error) => {
                self.add_error_attachment(&mut cucumber_events, &error, &uri);
                return GherkinEventsIter {
                    cucumber_events_iter: cucumber_events.into_iter(),
                };
            }
        };

        if self.print_source {
            cucumber_events.push(CucumberEvent::from(source_event));
        }

        let pickles: Vec<Pickle> = if self.print_pickles {
            self.compiler.compile(&gherkin_document)
                .into_iter()
                .map(Pickle::from)
                .collect()
        } else {
            Vec::new()
        };

        if self.print_ast {
            let uri = uri.clone();
            cucumber_events.push(CucumberEvent::from(GherkinDocumentEvent::new(uri, gherkin_document)));
        }

        for pickle in pickles {
            let uri = uri.clone();
            cucumber_events.push(CucumberEvent::from(PickleEvent::new(uri, pickle)));
        }

        GherkinEventsIter {
            cucumber_events_iter: cucumber_events.into_iter(),
        }
    }

    fn add_error_attachment(
        &self,
        cucumber_events: &mut Vec<CucumberEvent>,
        error: &Error,
        uri: &str,
    ) {
        match error {
            Error::Composite(composite_errors) => {
                for wrapped_error in composite_errors {
                    self.add_error_attachment(cucumber_events, &wrapped_error, uri);
                }
            }
            error_kind => {
                let error_location = error_kind
                    .get_location()
                    .unwrap_or_else(|| Location::new(0, 0));
                let event_location = attachment_event::Location::new(
                    error_location.line,
                    error_location.column,
                );
                let source_ref = attachment_event::SourceRef::new(uri.to_owned(), event_location);
                let attachment_event = AttachmentEvent::new(source_ref, error.to_string());
                cucumber_events.push(CucumberEvent::from(attachment_event));
            }
        }
    }
}

pub struct GherkinEventsIter {
    cucumber_events_iter: ::std::vec::IntoIter<CucumberEvent>,
}

impl Iterator for GherkinEventsIter {
    type Item = CucumberEvent;

    fn next(&mut self) -> Option<CucumberEvent> {
        self.cucumber_events_iter.next()
    }
}
