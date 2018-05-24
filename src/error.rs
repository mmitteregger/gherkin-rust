use std::fmt;
use std::io;
use std::result;

use failure::{Fail, Context, Backtrace};
use serde_json;

use ast::Location;
//use token::Token;

/// A crate private constructor for `Error`.
pub(crate) fn new_error(kind: ErrorKind) -> Error {
    Error(Context::new(kind))
}

/// A type alias for `Result<T, gherkin::Error>`.
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(Context<ErrorKind>);

impl Error {
    /// Return the specific type of this error.
    pub fn kind(&self) -> &ErrorKind {
        &*self.0.get_context()
    }
}

/// The specific type of an error.
#[derive(Fail, Debug)]
pub enum ErrorKind {
    /// An I/O error that occurred while reading CSV data.
    Io(io::Error),
    SerdeJson(serde_json::Error),
    AstBuilder {
        location: Option<Location>,
        message: String,
    },
    NoSuchLanguage {
        location: Option<Location>,
        language: String,
    },
    UnexpectedToken {
        location: Option<Location>,
        message: String,
        state_comment: String,
//        received_token: Token,
//        expected_tokens: Vec<String>,
    },
    UnexpectedEof {
        location: Option<Location>,
        message: String,
        state_comment: String,
//        expected_tokens: Vec<String>,
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
        Error(Context::new(ErrorKind::Io(err)))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error(Context::new(ErrorKind::SerdeJson(err)))
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error(Context::new(kind))
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(context: Context<ErrorKind>) -> Error {
        Error(context)
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.0.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.0.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Display for ErrorKind {
    #[allow(unused)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::Io(ref err) => err.fmt(f),
            ErrorKind::SerdeJson(ref err) => err.fmt(f),
            ErrorKind::AstBuilder { ref location, ref message } => {
                match *location {
                    Some(ref location) => write!(f, "({}:{}): {}",
                        location.get_line(), location.get_column(), message),
                    None => write!(f, "{}", message),
                }
            }
            ErrorKind::NoSuchLanguage { ref location, ref language } => {
                write!(f, "language not supported: {}", language)
            },
            ErrorKind::UnexpectedToken {
//                ref location, ref state_comment, ref received_token, ref expected_tokens,
                ref location, ref message, ref state_comment,
            } => {
//                let received = received_token.get_token_value().trim();
//                let expected = expected_tokens.join(", ");
//                write!(f, "expected: {}, got '{}'", expected, received)
                write!(f, "{}", message)
            },
//            ErrorKind::UnexpectedEof { ref location, ref state_comment, ref expected_tokens } => {
            ErrorKind::UnexpectedEof { ref location, ref message, ref state_comment } => {
//                let expected = expected_tokens.join(", ");
//                write!(f, "unexpected end of file, expected: {}", expected)
                write!(f, "{}", message)
            },
            ErrorKind::Composite(ref errors) => {
                write!(f, "multiple parse errors:")?;

                let separator = '\n';
                for error in errors {
                    write!(f, "{}* {}", separator, *error)?;
                }

                Ok(())
            },
            ErrorKind::__Nonexhaustive => unreachable!(),
        }
    }
}
