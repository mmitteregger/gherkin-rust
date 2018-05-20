use std::fs;

use event::SourceEvent;
use error::Result;

pub struct SourceEvents {
    paths: Vec<String>,
}

impl SourceEvents {
    pub fn new(paths: Vec<String>) -> SourceEvents {
        SourceEvents {
            paths,
        }
    }
}

pub struct SourceEventsIter {
    paths_iter: ::std::vec::IntoIter<String>,
}

impl IntoIterator for SourceEvents {
    type Item = Result<SourceEvent>;
    type IntoIter = SourceEventsIter;

    fn into_iter(self) -> SourceEventsIter {
        SourceEventsIter {
            paths_iter: self.paths.into_iter()
        }
    }
}

impl Iterator for SourceEventsIter {
    type Item = Result<SourceEvent>;

    fn next(&mut self) -> Option<Result<SourceEvent>> {
        match self.paths_iter.next() {
            Some(path) => {
                let data = match fs::read_to_string(&path) {
                    Ok(data) => data,
                    Err(error) => {
                        return Some(Err(error.into()));
                    },
                };
                Some(Ok(SourceEvent::new(path.to_owned(), data)))
            }
            None => None,
        }
    }
}
