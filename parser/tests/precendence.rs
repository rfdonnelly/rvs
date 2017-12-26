extern crate rvs_parser;

mod utils;
use utils::*;

#[test]
fn unary() {
    assert_eq!(
        parse("a = ~0 + 1;"),
        "[Assignment(Identifier(\"a\"), BinaryOperation(UnaryOperation(Neg, Number(0)), Add, Number(1)))]");
}
