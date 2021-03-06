use std::fmt;
use std::io;
use std::result;

use failure::Fail;

use crate::token::Token;
use crate::Location;

/// A type alias for `Result<T, gherkin::Error>`.
pub type Result<T> = result::Result<T, Error>;

/// The specific type of an error.
#[derive(Fail, Debug)]
pub enum Error {
    /// An I/O error that occurred while reading a feature file.
    Io(#[cause] io::Error),
    DocumentBuilder {
        location: Location,
        message: String,
    },
    NoSuchLanguage {
        location: Location,
        language: String,
    },
    UnexpectedToken {
        location: Location,
        state_comment: String,
        received_token: Box<Token>,
        expected_tokens: &'static [&'static str],
    },
    UnexpectedEof {
        location: Location,
        state_comment: String,
        expected_tokens: &'static [&'static str],
    },
    Composite(Vec<Error>),
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::DocumentBuilder {
                ref location,
                ref message,
            } => write!(f, "{}: {}", location, message),
            Error::NoSuchLanguage {
                ref location,
                ref language,
            } => write!(f, "{}: Language not supported: {}", location, language),
            Error::UnexpectedToken {
                ref location,
                ref received_token,
                ref expected_tokens,
                ..
            } => {
                let received = received_token.get_token_value().trim();
                let expected = expected_tokens.join(", ");
                write!(
                    f,
                    "{}: expected: {}, got '{}'",
                    location, expected, received
                )
            }
            Error::UnexpectedEof {
                ref location,
                ref expected_tokens,
                ..
            } => {
                let expected = expected_tokens.join(", ");
                write!(
                    f,
                    "{}: unexpected end of file, expected: {}",
                    location, expected
                )
            }
            Error::Composite(ref errors) => {
                write!(f, "multiple parse errors:")?;

                let separator = '\n';
                for error in errors {
                    write!(f, "{}* {}", separator, *error)?;
                }

                Ok(())
            }
            Error::__Nonexhaustive => unreachable!(),
        }
    }
}

impl Error {
    pub fn get_location(&self) -> Option<Location> {
        match *self {
            Error::Io(ref _err) => None,
            Error::DocumentBuilder { location, .. } => Some(location),
            Error::NoSuchLanguage { location, .. } => Some(location),
            Error::UnexpectedToken { location, .. } => Some(location),
            Error::UnexpectedEof { location, .. } => Some(location),
            Error::Composite(ref _errors) => None,
            Error::__Nonexhaustive => unreachable!(),
        }
    }
}
