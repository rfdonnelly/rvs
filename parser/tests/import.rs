extern crate rvs_parser;

use std::env::current_dir;

use rvs_parser::parse;
use rvs_parser::SearchPath;

/// Verify search path priority
#[test]
fn same_filename_different_directory() {
    let mut search_path = SearchPath::new();
    let fixtures = current_dir().unwrap().join("tests/import/same_filename_different_directory");
    search_path.add(&fixtures.join("a"));
    search_path.add(&fixtures.join("b"));

    let items = parse("import 'a.rvs';", &mut search_path).unwrap();
    assert_eq!(format!("{:?}", items),
        "[Assignment(Identifier(\"a\"), Number(0))]");
}

#[test]
fn source_relative() {
    let mut search_path = SearchPath::new();
    let fixtures = current_dir().unwrap().join("tests/import/source_relative");
    search_path.add(&fixtures);
    search_path.add(&fixtures.join("path"));

    let items = parse("import 'a.rvs';", &mut search_path).unwrap();
    assert_eq!(format!("{:?}", items),
        "[Assignment(Identifier(\"c\"), Number(0)), Assignment(Identifier(\"b\"), Number(0)), Assignment(Identifier(\"a\"), Number(0))]");
}

#[test]
fn import_is_idempotent() {
    let mut search_path = SearchPath::new();
    let fixtures = current_dir().unwrap().join("tests/import/import_is_idempotent");
    search_path.add(&fixtures);

    let items = parse("import 'a.rvs';", &mut search_path).unwrap();
    assert_eq!(format!("{:?}", items),
        "[Assignment(Identifier(\"a\"), Number(1)), Assignment(Identifier(\"a\"), Number(2))]");
}

mod error {
    use super::*;

    #[test]
    fn not_in_search_path() {
        let mut search_path = SearchPath::new();
        let fixtures = current_dir().unwrap().join("tests/import");
        search_path.add(&fixtures);

        assert!(parse("import 'a.rvs';", &mut search_path).is_err());
    }
}
