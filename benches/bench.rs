#![feature(test)]

extern crate test;
extern crate gherkin;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use std::path::{Path, PathBuf};
    use std::fs;

    use gherkin::Parser;
    use gherkin::event;

    #[bench]
    fn bench_parser_good_features(bencher: &mut Bencher) {
        let features = read_features("good");
        let mut parser = Parser::default();

        bencher.iter(|| {
            for feature in &features {
                parser.parse_str(feature).unwrap();
            }
        });
    }

    #[bench]
    fn bench_parser_bad_features(bencher: &mut Bencher) {
        let features = read_features("bad");
        let mut parser = Parser::default();

        bencher.iter(|| {
            for feature in &features {
                parser.parse_str(feature).unwrap_err();
            }
        });
    }

    #[bench]
    fn bench_compiler_good_features(bencher: &mut Bencher) {
        let features = read_features("good");

        bencher.iter(|| {
            for feature in &features {
                let data = feature.clone();
                event::generate(data, "bench_compiler_good_features").unwrap();
            }
        });
    }

    #[bench]
    fn bench_compiler_bad_features(bencher: &mut Bencher) {
        let features = read_features("bad");

        bencher.iter(|| {
            for feature in &features {
                let data = feature.clone();
                event::generate(data, "bench_compiler_bad_features").unwrap_err();
            }
        });
    }

    fn read_features<P: AsRef<Path>>(path: P) -> Vec<String> {
        fs::read_dir(PathBuf::from("testdata").join(path))
            .unwrap()
            .into_iter()
            .map(Result::unwrap)
            .filter(|entry| entry.file_name().to_string_lossy().ends_with(".feature"))
            .map(|entry| fs::read_to_string(entry.path()).unwrap())
            .collect::<Vec<String>>()
    }
}
