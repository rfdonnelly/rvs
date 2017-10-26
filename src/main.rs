extern crate rand;

mod ast;
mod sequence;
mod grammar;

use grammar::*;
use sequence::sequence_from_ast;
use sequence::Sequence;

fn main() {
}

fn eval_by_ast(s: &str) -> u32 {
    match expr(s) {
        Ok(ast) => ast.eval(),
        Err(_) => panic!("Could not parse: '{}'", s),
    }
}

fn parse(s: &str) -> Box<Sequence> {
    match expr(s) {
        Ok(ast) => {
            sequence_from_ast(&ast)
        },
        Err(_) => panic!("Could not parse: '{}'", s),
    }
}

mod tests {
    mod eval_by_ast {
        use super::super::*;

        #[test]
        fn basic() {
            assert_eq!(eval_by_ast("1+2*3"), 7);
        }
    }

    mod eval_by_sequence {
        use super::super::*;

        #[test]
        fn basic() {
            let mut sequence = parse("1+2*3");
            assert_eq!(sequence.next(), 7);
        }

        #[test]
        fn range() {
            use std::collections::HashMap;

            let mut sequence = parse("1+[0,1]");

            let mut values = HashMap::new();
            for _ in 0..100 {
                let value = sequence.next();
                let entry = values.entry(value).or_insert(0);
                *entry += 1;
                assert!(value == 1 || value == 2);
            }

            assert!(values[&1] > 0);
            assert!(values[&2] > 0);
        }
    }
}

