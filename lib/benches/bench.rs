#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use test::Bencher;

    use cucumber_messages::id_generator::IncrementingIdGenerator;

    use gherkin::{self, DocumentBuilder, IncludeOptions, Parser};

    #[bench]
    fn bench_parser_good_features(bencher: &mut Bencher) {
        let features = read_features("good");
        let mut id_generator = IncrementingIdGenerator::new();
        let builder = DocumentBuilder::with_id_generator(&mut id_generator);
        let mut parser = Parser::with_builder(builder);

        bencher.iter(|| {
            for feature in &features {
                parser.parse_str(feature).unwrap();
            }
        });
    }

    #[bench]
    fn bench_parser_bad_features(bencher: &mut Bencher) {
        let features = read_features("bad");
        let mut id_generator = IncrementingIdGenerator::new();
        let builder = DocumentBuilder::with_id_generator(&mut id_generator);
        let mut parser = Parser::with_builder(builder);

        bencher.iter(|| {
            for feature in &features {
                parser.parse_str(feature).unwrap_err();
            }
        });
    }

    #[bench]
    fn bench_compiler_good_features(bencher: &mut Bencher) {
        let feature_paths = feature_paths_iter("good").collect::<Vec<PathBuf>>();

        bencher.iter(|| {
            let mut id_generator = IncrementingIdGenerator::new();
            gherkin::parse_paths(
                &feature_paths,
                IncludeOptions {
                    source: false,
                    gherkin_document: false,
                    pickles: true,
                },
                &mut id_generator,
            )
            .unwrap();
        });
    }

    #[bench]
    fn bench_compiler_bad_features(bencher: &mut Bencher) {
        let feature_paths = feature_paths_iter("bad").collect::<Vec<PathBuf>>();

        bencher.iter(|| {
            let mut id_generator = IncrementingIdGenerator::new();
            gherkin::parse_paths(
                &feature_paths,
                IncludeOptions {
                    source: false,
                    gherkin_document: false,
                    pickles: true,
                },
                &mut id_generator,
            )
            .unwrap();
        });
    }

    fn read_features<P: AsRef<Path>>(path: P) -> Vec<String> {
        feature_paths_iter(path)
            .map(fs::read_to_string)
            .map(Result::unwrap)
            .collect()
    }

    fn feature_paths_iter<P: AsRef<Path>>(path: P) -> impl Iterator<Item = PathBuf> {
        fs::read_dir(PathBuf::from("../testdata").join(path))
            .unwrap()
            .into_iter()
            .map(Result::unwrap)
            .filter(|entry| entry.file_name().to_string_lossy().ends_with(".feature"))
            .map(|entry| entry.path())
    }
}
