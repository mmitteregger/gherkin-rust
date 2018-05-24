use std::io::prelude::*;
use std::io::BufReader;
use error::{Result};
use ast::Location;
use token::Token;
use gherkin_line::GherkinLine;
use parser::TokenScan;

/// The scanner reads a gherkin doc (typically read from a .feature file)
/// and creates a token for each line.
/// The tokens are passed to the parser, which outputs an AST (Abstract Syntax Tree).
///
/// If the scanner sees a # language header, it will reconfigure itself dynamically to look for
/// Gherkin keywords for the associated language.
/// The keywords are defined in gherkin-languages.json.
pub struct TokenScanner<R> {
    reader: BufReader<R>,
    line_number: usize,
}

impl<R: Read> From<R> for TokenScanner<R> {
    fn from(source: R) -> Self {
        TokenScanner {
            reader: BufReader::new(source),
            line_number: 0,
        }
    }
}

impl<R: Read> TokenScan for TokenScanner<R> {
    fn scan_next_token(&mut self) -> Result<Token> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;

        let is_eof = line.is_empty();

        // rusts BufReader::read_line function includes the newline delimiter,
        // so it has to be removed
        if line.ends_with('\n') {
            line.pop();
        }

        self.line_number += 1;

        let location = Location::new(self.line_number, 0);

        let token = if is_eof {
            Token::new(None, Some(location))
        } else {
            let gherkin_line = GherkinLine::new(line);
            Token::new(Some(gherkin_line), Some(location))
        };
        Ok(token)
    }
}