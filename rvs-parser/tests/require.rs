extern crate rvs_parser;

use std::env::current_dir;

use rvs_parser::parse;
use rvs_parser::RequirePaths;

/// Verify search path priority
#[test]
fn same_filename_different_directory() {
    let mut require_paths = RequirePaths::new();
    let fixtures = current_dir().unwrap().join("fixtures/require/same_filename_different_directory");
    require_paths.add_search_path(&fixtures.join("a"));
    require_paths.add_search_path(&fixtures.join("b"));

    let items = parse("require 'a.rvs';", &mut require_paths).unwrap();
    assert_eq!(format!("{:?}", items),
        "[Multiple([Single(Assignment(Identifier(\"a\"), Number(0)))])]");
}

#[test]
fn source_relative() {
    let mut require_paths = RequirePaths::new();
    let fixtures = current_dir().unwrap().join("fixtures/require/source_relative");
    require_paths.add_search_path(&fixtures);
    require_paths.add_search_path(&fixtures.join("path"));

    let items = parse("require 'a.rvs';", &mut require_paths).unwrap();
    assert_eq!(format!("{:?}", items),
        "[Multiple([Multiple([Multiple([Single(Assignment(Identifier(\"c\"), Number(0)))]), Single(Assignment(Identifier(\"b\"), Number(0)))]), Single(Assignment(Identifier(\"a\"), Number(0)))])]");
}

#[test]
fn require_is_idempotent() {
    let mut require_paths = RequirePaths::new();
    let fixtures = current_dir().unwrap().join("fixtures/require/require_is_idempotent");
    require_paths.add_search_path(&fixtures);

    let items = parse("require 'a.rvs';", &mut require_paths).unwrap();
    assert_eq!(format!("{:?}", items),
        "[Multiple([Multiple([Single(Assignment(Identifier(\"a\"), Number(1)))]), Single(Assignment(Identifier(\"a\"), Number(2))), Multiple([])])]");
}
