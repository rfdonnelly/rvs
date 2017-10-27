extern crate rand;

mod ast;
mod grammar;
mod sequences;

use std::collections::HashMap;

use sequences::sequence_from_ast;
use sequences::sequences_from_ast;
use sequences::Sequence;

fn main() {
}

fn eval_by_ast(s: &str) -> u32 {
    match grammar::expr(s) {
        Ok(ast) => ast.eval(),
        Err(_) => panic!("Could not parse: '{}'", s),
    }
}

fn parse_expression(s: &str) -> Box<Sequence> {
    match grammar::expr(s) {
        Ok(ast) => {
            sequence_from_ast(&ast)
        },
        Err(_) => panic!("Could not parse: '{}'", s),
    }
}

fn parse_assignments(s: &str) -> HashMap<String, Box<Sequence>> {
    match grammar::assignments(s) {
        Ok(assignments) => {
            sequences_from_ast(assignments)
        },
        Err(_) => panic!("Could not parse: '{}'", s),
    }
}

#[cfg(test)]
mod tests {
    mod eval_by_ast {
        use super::super::*;

        #[test]
        fn basic() {
            assert_eq!(eval_by_ast("1+2*3"), 7);
        }
    }

    mod parse_expression {
        use super::super::*;

        #[test]
        fn basic() {
            let mut sequence = parse_expression("1+2*3");
            assert_eq!(sequence.next(), 7);
        }

        #[test]
        fn range() {
            use std::collections::HashMap;

            let mut sequence = parse_expression("1+[0,1]");

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

    mod parse_assignments {
        use super::super::*;

        use std::collections::hash_map::Entry::Occupied;

        #[test]
        fn basic() {
            let mut sequences = parse_assignments("a=[0,1];\nb=2;");

            assert!(sequences.contains_key("a"));
            assert!(sequences.contains_key("b"));

            if let Occupied(mut entry) = sequences.entry("a".into()) {
                let value = entry.get_mut().next();
                assert!(value == 0 || value == 1);
            }
            if let Occupied(mut entry) = sequences.entry("b".into()) {
                assert_eq!(entry.get_mut().next(), 2);
            }
        }
    }
}

