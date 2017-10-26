extern crate rand;

mod ast;
mod sequence;
mod grammar;

use grammar::*;
use sequence::sequence_from_ast;

fn main() {
}

fn eval_by_ast(s: &str) -> u32 {
    match expr(s) {
        Ok(ast) => ast.eval(),
        Err(_) => panic!("Could not parse: '{}'", s),
    }
}

fn eval_by_sequence(s: &str) -> u32 {
    match expr(s) {
        Ok(ast) =>  {
            let mut sequence = sequence_from_ast(&ast);
            sequence.next()
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
            assert_eq!(eval_by_sequence("1+2*3"), 7);
        }
    }
}

