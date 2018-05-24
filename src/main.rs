extern crate gherkin;
extern crate serde_json;

use std::env;
use std::io::{self, Write};

use gherkin::Result;
use gherkin::stream::*;

fn main() -> Result<()> {
    // TODO: Should we use clap or structopt?
    let mut print_source = true;
    let mut print_ast = true;
    let mut print_pickles = true;
    let mut paths = Vec::new();

    for arg in env::args().skip(1) {
        match arg.as_ref() {
            "--no-source" => print_source = false,
            "--no-ast" => print_ast = false,
            "--no-pickles" => print_pickles = false,
            _ => paths.push(arg),
        }
    }

    let source_events = SourceEvents::new(paths);
    let mut gherkin_events = GherkinEvents::new(print_source, print_ast, print_pickles);

    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();

    for source_event_result in source_events {
        let source_event = source_event_result?;

        for cucumber_event in gherkin_events.iter_source_event(source_event) {
            serde_json::to_writer(&mut stdout_handle, &cucumber_event)?;
            stdout_handle.write_all(b"\n")?;
        }
    }

    stdout_handle.flush()?;
    Ok(())
}
