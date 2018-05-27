extern crate gherkin;

use std::env;
use std::fs::File;
use std::io::{self, Write};

use gherkin::{Parser, Result, TokenFormatterBuilder};

fn main() -> Result<()> {
    // TODO: Use clap or structopt?
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();

    let mut parser = Parser::with_builder(TokenFormatterBuilder::default());

    for file_name in env::args().skip(1) {
        let file = File::open(&file_name)?;
        let result = parser.parse_reader(file)?;
        stdout_handle.write_all(result.as_bytes())?;
    }

    stdout_handle.flush()?;
    Ok(())
}
