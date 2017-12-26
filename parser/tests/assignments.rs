extern crate rvs_parser;

mod utils;
use utils::*;

#[test]
fn good() {
    assert!(parse_result("a=5;").is_ok());
}

#[test]
fn ast() {
    assert_eq!(
        parse("a=5;"),
        "[Assignment(Identifier(\"a\"), Number(5))]");
}

#[test]
fn bad() {
    assert!(parse_result("a=5").is_err());
}

#[test]
fn with_enum() {
    assert!(parse_result("a = Enum::Value;").is_ok());
}
