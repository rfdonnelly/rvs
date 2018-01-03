extern crate rvs_parser;

mod utils;
use utils::*;

#[test]
fn good() {
    assert!(parse_result("a = 0;").is_ok());
    assert!(parse_result("a_ = 0;").is_ok());
    assert!(parse_result("__ = 0;").is_ok());
    assert!(parse_result("_0 = 0;").is_ok());
    assert!(parse_result("a::B = 0;").is_ok());
}

#[test]
fn bad() {
    assert!(parse_result("a-b = 0;").is_err());
    assert!(parse_result("0b = 0;").is_err());
    assert!(parse_result("1_ = 0;").is_err());
}
