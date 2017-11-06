extern crate rand;
extern crate libc;

mod ast;
mod grammar;
mod sequences;
pub mod c_api;

use std::collections::HashMap;

use sequences::sequence_from_ast;
use sequences::sequences_from_ast;
use sequences::Sequence;

use grammar::ParseResult;

fn parse_expression(s: &str) -> Box<Sequence> {
    match grammar::expr(s) {
        Ok(ast) => {
            sequence_from_ast(&ast)
        },
        Err(_) => panic!("Could not parse: '{}'", s),
    }
}

fn parse_assignments(s: &str, ids: &mut HashMap<String, usize>, sequences: &mut Vec<Box<Sequence>>) -> ParseResult<()> {
    match grammar::assignments(s) {
        Ok(assignments) => {
            sequences_from_ast(assignments, ids, sequences);

            Ok(())
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
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
            let mut ids = HashMap::new();
            let mut sequences = Vec::new();
            assert!(parse_assignments("a=[0,1];\nb=2;", &mut ids, &mut sequences).is_ok());

            assert!(ids.contains_key("a"));
            assert!(ids.contains_key("b"));

            if let Occupied(entry) = ids.entry("a".into()) {
                let id = entry.get();
                let value = sequences[*id].next();
                assert!(value == 0 || value == 1);
            }
            if let Occupied(entry) = ids.entry("b".into()) {
                let id = entry.get();
                let value = sequences[*id].next();
                assert_eq!(value, 2);
            }
        }
    }
}

