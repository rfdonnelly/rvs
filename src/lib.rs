extern crate rand;
extern crate libc;

mod ast;
mod grammar;
mod types;
pub mod c_api;

use std::collections::HashMap;

use types::rvs_from_ast;
use types::Rv;

use grammar::ParseResult;

fn parse_assignments(s: &str, ids: &mut HashMap<String, usize>, variables: &mut Vec<Box<Rv>>) -> ParseResult<()> {
    match grammar::assignments(s) {
        Ok(assignments) => {
            rvs_from_ast(assignments, ids, variables);

            Ok(())
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    mod parse_assignments {
        use super::super::*;

        use std::collections::hash_map::Entry::Occupied;

        #[test]
        fn basic() {
            let mut ids = HashMap::new();
            let mut variables = Vec::new();
            assert!(parse_assignments("a=[0,1];\nb=2;", &mut ids, &mut variables).is_ok());

            assert!(ids.contains_key("a"));
            assert!(ids.contains_key("b"));

            if let Occupied(entry) = ids.entry("a".into()) {
                let id = entry.get();
                let value = variables[*id].next();
                assert!(value == 0 || value == 1);
            }
            if let Occupied(entry) = ids.entry("b".into()) {
                let id = entry.get();
                let value = variables[*id].next();
                assert_eq!(value, 2);
            }
        }
    }
}

