extern crate gherkin;

use std::env;
use std::fs::File;
use std::io::{self, Write};
use gherkin::{Result, TokenFormatterBuilder, Parser};

fn main() -> Result<()> {
    // TODO: Should we use clap or structopt?
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();

    let token_formatter_builder = TokenFormatterBuilder::default();
    let mut parser = Parser::with_builder(token_formatter_builder);

    for file_name in env::args().skip(1) {
        let file = File::open(&file_name)?;
        let result = parser.parse_reader(file)?;
        stdout_handle.write_all(result.as_bytes())?;
    }

    stdout_handle.flush()?;
    Ok(())
}
