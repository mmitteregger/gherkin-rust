use std::fs;
use std::io;
use std::path::Path;

pub use cucumber_messages::ast;
pub use cucumber_messages::id_generator::{IdGenerator, IncrementingIdGenerator};
pub use cucumber_messages::pickle;

use cucumber_messages::pickle::Pickle;
use cucumber_messages::source::{Source, SourceReference, SourceReferenceType};
use cucumber_messages::{Envelope, Message, ParseError};

pub use crate::document_builder::DocumentBuilder;
pub use crate::error::{Error, Result};
pub use crate::location::Location;
pub use crate::parser::{DialectProvider, Parser, ParserOptions};
pub use crate::token_formatter_builder::TokenFormatterBuilder;

mod ast_node;
mod constant;
pub mod cuke;
mod dialect;
mod dialect_provider;
mod document_builder;
mod error;
mod line;
mod location;
mod parser;
mod token;
mod token_formatter_builder;
mod token_matcher;
mod token_scanner;

pub struct IncludeOptions {
    pub source: bool,
    pub gherkin_document: bool,
    pub pickles: bool,
}

pub fn parse_paths<P>(
    paths: P,
    include_options: IncludeOptions,
    id_generator: &mut dyn IdGenerator,
) -> io::Result<Vec<Envelope>>
where
    P: IntoIterator,
    P::Item: AsRef<Path>,
{
    let mut messages = Vec::new();

    let builder = DocumentBuilder::with_id_generator(id_generator);
    let mut parser = Parser::with_builder(builder);

    for path in paths {
        let envelope = create_envelope_from_path(path.as_ref())?;

        if include_options.source {
            messages.push(envelope);
            messages.extend(parse_envelope(
                &mut parser,
                &include_options,
                messages.last().unwrap(),
            )?);
        } else {
            messages.extend(parse_envelope(&mut parser, &include_options, &envelope)?);
        }
    }

    Ok(messages)
}

fn create_envelope_from_path(path: &Path) -> io::Result<Envelope> {
    let data = fs::read_to_string(path)?;
    let envelope = create_source_envelope(data, path);
    Ok(envelope)
}

fn create_source_envelope(data: String, path: &Path) -> Envelope {
    Envelope {
        message: Some(Message::Source(Source {
            data,
            uri: path.display().to_string(),
            media_type: String::from("text/x.cucumber.gherkin+plain"),
        })),
    }
}

fn parse_envelope(
    parser: &mut Parser<DocumentBuilder>,
    include_options: &IncludeOptions,
    envelope: &Envelope,
) -> io::Result<Vec<Envelope>> {
    let mut messages = Vec::new();

    let source = match &envelope.message {
        Some(message) => {
            if let Message::Source(source) = message {
                source
            } else {
                return Ok(messages);
            }
        }
        None => return Ok(messages),
    };

    if include_options.gherkin_document || include_options.pickles {
        let mut gherkin_document = match parser.parse_str(&source.data) {
            Ok(gherkin_document) => gherkin_document,
            Err(error) => {
                add_error_attachments(&mut messages, error, &source.uri)?;
                return Ok(messages);
            }
        };

        gherkin_document.uri = source.uri.to_string();

        let pickles = if include_options.pickles {
            let builder = parser.builder_mut();
            let id_generator = builder.id_generator_mut();
            let mut compiler = crate::cuke::Compiler::new(id_generator);

            let cukes = compiler.compile(&gherkin_document);
            cukes.into_iter().map(Pickle::from).collect::<Vec<Pickle>>()
        } else {
            Vec::new()
        };

        if include_options.gherkin_document {
            messages.push(Envelope {
                message: Some(Message::GherkinDocument(gherkin_document)),
            });
        }

        for pickle in pickles {
            messages.push(Envelope {
                message: Some(Message::Pickle(pickle)),
            });
        }
    }

    Ok(messages)
}

fn add_error_attachments(messages: &mut Vec<Envelope>, error: Error, uri: &str) -> io::Result<()> {
    match error {
        Error::Composite(composite_error) => {
            for error in composite_error {
                add_error_attachments(messages, error, uri)?;
            }
            Ok(())
        }
        Error::DocumentBuilder { location, .. }
        | Error::NoSuchLanguage { location, .. }
        | Error::UnexpectedToken { location, .. }
        | Error::UnexpectedEof { location, .. } => {
            messages.push(create_parse_error_envelope(&error, uri, location));
            Ok(())
        }
        Error::Io(io_error) => Err(io_error),
        Error::__Nonexhaustive => unreachable!(),
    }
}

fn create_parse_error_envelope(error: &Error, uri: &str, location: Location) -> Envelope {
    Envelope {
        message: Some(Message::ParseError(ParseError {
            source: Some(create_source_reference(uri, location)),
            message: error.to_string(),
        })),
    }
}

fn create_source_reference(uri: &str, location: Location) -> SourceReference {
    SourceReference {
        location: Some(ast::Location::from(location)),
        reference: Some(SourceReferenceType::Uri(uri.to_string())),
    }
}
