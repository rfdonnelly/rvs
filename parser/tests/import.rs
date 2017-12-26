extern crate rvs_parser;

use std::env::current_dir;

use rvs_parser::parse;
use rvs_parser::ImportPaths;

/// Verify search path priority
#[test]
fn same_filename_different_directory() {
    let mut import_paths = ImportPaths::new();
    let fixtures = current_dir().unwrap().join("tests/import/same_filename_different_directory");
    import_paths.add_search_path(&fixtures.join("a"));
    import_paths.add_search_path(&fixtures.join("b"));

    let items = parse("import 'a.rvs';", &mut import_paths).unwrap();
    assert_eq!(format!("{:?}", items),
        "[Assignment(Identifier(\"a\"), Number(0))]");
}

#[test]
fn source_relative() {
    let mut import_paths = ImportPaths::new();
    let fixtures = current_dir().unwrap().join("tests/import/source_relative");
    import_paths.add_search_path(&fixtures);
    import_paths.add_search_path(&fixtures.join("path"));

    let items = parse("import 'a.rvs';", &mut import_paths).unwrap();
    assert_eq!(format!("{:?}", items),
        "[Assignment(Identifier(\"c\"), Number(0)), Assignment(Identifier(\"b\"), Number(0)), Assignment(Identifier(\"a\"), Number(0))]");
}

#[test]
fn import_is_idempotent() {
    let mut import_paths = ImportPaths::new();
    let fixtures = current_dir().unwrap().join("tests/import/import_is_idempotent");
    import_paths.add_search_path(&fixtures);

    let items = parse("import 'a.rvs';", &mut import_paths).unwrap();
    assert_eq!(format!("{:?}", items),
        "[Assignment(Identifier(\"a\"), Number(1)), Assignment(Identifier(\"a\"), Number(2))]");
}

mod error {
    use super::*;

    #[test]
    fn not_in_search_path() {
        let mut import_paths = ImportPaths::new();
        let fixtures = current_dir().unwrap().join("tests/import");
        import_paths.add_search_path(&fixtures);

        assert!(parse("import 'a.rvs';", &mut import_paths).is_err());
    }
}
