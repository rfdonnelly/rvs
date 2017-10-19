mod ast;

mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

use ast::Node;
use ast::Opcode;
use ast::eval_walk;
use grammar::*;

use std::ops::Deref;

fn main() {
}

fn eval(expr: &str) -> u32 {
    match infix_arith(expr) {
        Ok(ast) => eval_walk(ast.deref()),
        Err(_) => panic!("Could not parse: '{}'", expr),
    }
}

mod tests {
    mod eval {
        use super::super::*;

        #[test]
        fn basic() {
            assert_eq!(eval("1+2*3"), 7);
        }
    }

    mod atom {
        use super::super::*;

        #[test]
        fn good() {
            assert_eq!(atom("(5)"), Ok(Box::new(Node::Number(5))));
            assert_eq!(atom("5"), Ok(Box::new(Node::Number(5))));
            assert_eq!(atom("0xa"), Ok(Box::new(Node::Number(10))));
            assert_eq!(atom("0xaf"), Ok(Box::new(Node::Number(0xaf))));
        }

        #[test]
        fn bad() {
            assert!(atom("(5))").is_err());
            assert!(atom("(5").is_err());
        }
    }

    mod infix_arith {
        use super::super::*;

        #[test]
        fn good() {
            assert_eq!(infix_arith("1+2"), Ok(
                Box::new(Node::Operation(
                    Box::new(Node::Number(1)),
                    Opcode::Add,
                    Box::new(Node::Number(2))
                ))
            ));

            assert_eq!(format!("{:?}", infix_arith("1+2*3")),
            "Ok(Operation(Number(1), Add, Operation(Number(2), Multiply, Number(3))))");
        }
    }

    mod id {
        use grammar::*;

        #[test]
        fn good() {
            assert!(id("a").is_ok());
            assert!(id("a_").is_ok());
            assert!(id("__").is_ok());
            assert!(id("_0").is_ok());
            assert!(id("a::B").is_ok());
        }

        #[test]
        fn bad() {
            assert!(id("a-b").is_err());
            assert!(id("0b").is_err());
            assert!(id("1_").is_err());
        }
    }

    mod number {
        use grammar::*;

        #[test]
        fn good() {
            assert!(number("5").is_ok());
            assert!(number("5_").is_ok());
            assert!(number("5_6").is_ok());
            assert!(number("5__6").is_ok());
        }

        #[test]
        fn bad() {
            assert!(number("a").is_err());
            assert!(number("_5").is_err());
        }
    }

    mod hex_number {
        use grammar::*;

        #[test]
        fn good() {
            assert!(hex_number("0xa5E4").is_ok());
            assert!(hex_number("0XA5").is_ok());
            assert!(hex_number("0X_A5").is_ok());
            assert!(hex_number("0XA_5").is_ok());
            assert!(hex_number("0XA__5").is_ok());
            assert!(hex_number("0XA5_").is_ok());
        }

        #[test]
        fn bad() {
            assert!(hex_number("0xg5").is_err());
        }
    }
}
