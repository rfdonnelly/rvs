extern crate rand;
extern crate libc;

mod ast;
mod grammar;
mod types;
pub mod c_api;


use types::rvs_from_ast;
use types::Context;

use grammar::ParseResult;

fn parse_assignments(s: &str, context: &mut Context) -> ParseResult<()> {
    match grammar::assignments(s) {
        Ok(assignments) => {
            rvs_from_ast(assignments, context);

            Ok(())
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::hash_map::Entry::Occupied;

    mod parse_assignments {
        use super::*;

        #[test]
        fn basic() {
            let mut context = Context::new();
            assert!(parse_assignments("a=[0,1];\nb=2;", &mut context).is_ok());

            assert!(context.handles.contains_key("a"));
            assert!(context.handles.contains_key("b"));

            if let Occupied(entry) = context.handles.entry("a".into()) {
                let id = entry.get();
                let value = context.variables[*id].next();
                assert!(value == 0 || value == 1);
            }
            if let Occupied(entry) = context.handles.entry("b".into()) {
                let id = entry.get();
                let value = context.variables[*id].next();
                assert_eq!(value, 2);
            }
        }
    }
}

