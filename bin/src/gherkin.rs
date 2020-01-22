use std::borrow::Cow;
use std::collections::VecDeque;
use std::env;
use std::io;
use std::path::Path;

use cucumber_messages::id_generator::IncrementingIdGenerator;
use cucumber_messages::io::{MessageWriter, NdjsonWriter};

use gherkin::{IncludeOptions, Result};

fn main() -> Result<()> {
    let stdout = io::stdout();
    let stdout_handle = stdout.lock();

    let mut paths = Vec::new();

    let mut include_options = IncludeOptions {
        source: true,
        gherkin_document: true,
        pickles: true,
    };

    let mut format: Cow<'static, str> = Cow::Borrowed("protobuf");
    let mut id_generator = None;

    let mut args = env::args().skip(1).collect::<VecDeque<String>>();

    while let Some(arg) = args.pop_front() {
        match arg.as_str() {
            "--no-source" => include_options.source = false,
            "--no-ast" => include_options.gherkin_document = false,
            "--no-pickles" => include_options.pickles = false,
            "--format" => format = Cow::Owned(args.pop_front().expect("format arg")),
            "--predictable-ids" => id_generator = Some(IncrementingIdGenerator::new()),
            _ => paths.push(arg),
        }
    }

    let paths_iter = paths.iter().map(|path_str| Path::new(path_str));

    let mut id_generator = match id_generator {
        Some(id_generator) => id_generator,
        None => unimplemented!("uuid generator"),
    };

    let mut message_writer = match format.as_ref() {
        "ndjson" => NdjsonWriter::new(stdout_handle),
        "protobuf" => unimplemented!("protobuf message writer"),
        _ => panic!("format needs to be either ndjson or protobuf (default)"),
    };

    let messages = gherkin::parse_paths(paths_iter, include_options, &mut id_generator).unwrap();
    for message in messages {
        message_writer.write(&message).unwrap();
    }

    message_writer.flush()?;
    Ok(())
}
