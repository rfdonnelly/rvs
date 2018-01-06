extern crate rvs_parser;

mod utils;
use utils::*;

#[test]
fn unary_inv() {
    assert_eq!(
        parse("a = ~0 + 1;"),
        "[Variable(\"a\", BinaryOperation(UnaryOperation(Inv, Number(0)), Add, Number(1)))]");
}

#[test]
fn unary_neg() {
    assert_eq!(
        parse("a = 4-1;"),
        "[Variable(\"a\", BinaryOperation(Number(4), Sub, Number(1)))]");

    assert_eq!(
        parse("a = 4+-1;"),
        "[Variable(\"a\", BinaryOperation(Number(4), Add, UnaryOperation(Neg, Number(1))))]");

    assert_eq!(
        parse("a = -1;"),
        "[Variable(\"a\", UnaryOperation(Neg, Number(1)))]");
}
