extern crate rvs;

use std::env::current_dir;

use rvs::parse;
use rvs::types::Context;

/// Verify search path priority
#[test]
fn same_filename_different_directory() {
    let mut context = Context::new();
    let fixtures = current_dir().unwrap().join("fixtures/require/same_filename_different_directory");
    context.requires.add_search_path(&fixtures.join("a"));
    context.requires.add_search_path(&fixtures.join("b"));

    parse("require 'a.rvs';", &mut context).unwrap();

    assert!(context.get("a").is_some());
    assert!(context.get("b").is_none());
}

#[test]
fn source_relative() {
    let mut context = Context::new();
    let fixtures = current_dir().unwrap().join("fixtures/require/source_relative");
    context.requires.add_search_path(&fixtures);
    context.requires.add_search_path(&fixtures.join("path"));

    parse("require 'a.rvs';", &mut context).unwrap();

    assert!(context.get("a").is_some());
    assert!(context.get("b").is_some());
}

#[test]
fn require_is_idempotent() {
    let mut context = Context::new();
    let fixtures = current_dir().unwrap().join("fixtures/require/require_is_idempotent");
    context.requires.add_search_path(&fixtures);

    parse("require 'a.rvs';", &mut context).unwrap();

    assert_eq!(context.get("a").unwrap().next(), 2);
}
