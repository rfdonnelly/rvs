use std::env::current_dir;

use rvs_parser::{Parser, SearchPath};

/// Verify search path priority
#[test]
fn same_filename_different_directory() {
    let fixtures = current_dir()
        .unwrap()
        .join("tests/import/same_filename_different_directory");
    let search_path = SearchPath::new(vec![fixtures.join("a"), fixtures.join("b")]);
    let parser = Parser::new(search_path);
    let items = parser.parse("import a;").unwrap();
    assert_eq!(format!("{:?}", items), "[Variable(\"a\", Number(0))]");
}

#[test]
fn source_relative() {
    let fixtures = current_dir().unwrap().join("tests/import/source_relative");
    let search_path = SearchPath::new(vec![fixtures.clone(), fixtures.join("path")]);
    let parser = Parser::new(search_path);
    let items = parser.parse("import a;").unwrap();
    assert_eq!(
        format!("{:?}", items),
        "[Variable(\"c\", Number(0)), Variable(\"b\", Number(0)), Variable(\"a\", Number(0))]"
    );
}

#[test]
fn import_is_idempotent() {
    let fixtures = current_dir()
        .unwrap()
        .join("tests/import/import_is_idempotent");
    let search_path = SearchPath::new(vec![fixtures]);
    let parser = Parser::new(search_path);
    let items = parser.parse("import a;").unwrap();
    assert_eq!(
        format!("{:?}", items),
        "[Variable(\"a\", Number(1)), Variable(\"a\", Number(2))]"
    );
}

mod error {
    use super::*;

    #[test]
    fn not_in_search_path() {
        let fixtures = current_dir().unwrap().join("tests/import");
        let search_path = SearchPath::new(vec![fixtures]);
        let parser = Parser::new(search_path);
        assert!(parser.parse("import a;").is_err());
    }
}
