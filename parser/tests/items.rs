extern crate rvs_parser;

mod utils;
use utils::*;

#[test]
fn good() {
    assert!(parse_result(" a  = 5 ; \nb=6;").is_ok());
}

#[test]
fn expr_whitespace() {
    assert!(parse_result("a = 5 + 6 | 10 * ( 5 ^ 3) ;").is_ok());
}

#[test]
fn ast() {
    assert_eq!(
        parse(" a  = // comment0\n5 ; // comment1\nb=6;"),
        "[Variable(\"a\", Number(5)), Variable(\"b\", Number(6))]"
    );
}
