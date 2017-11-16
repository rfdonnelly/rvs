extern crate rand;
extern crate libc;
extern crate linked_hash_map;

mod ast;
mod grammar;
mod types;
pub mod c_api;


use types::rvs_from_ast;
use types::Context;

use grammar::ParseResult;

fn parse_assignments(s: &str, context: &mut Context) -> ParseResult<()> {
    match grammar::items(s) {
        Ok(items) => {
            rvs_from_ast(&items, context);

            Ok(())
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use linked_hash_map::Entry::Occupied;

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

    mod display {
        use super::*;

        #[test]
        fn multiple() {
            let mut context = Context::new();
            assert!(parse_assignments("a=[0,1];b=[2,3];", &mut context).is_ok());
            assert_eq!(context.to_string(), "a = [0x0, 0x1];\nb = [0x2, 0x3];\n");
        }

        #[test]
        fn precendence() {
            let mut context = Context::new();
            assert!(parse_assignments("a = (10 + 6) * 8;", &mut context).is_ok());
            assert_eq!(context.to_string(), "a = ((0xa + 0x6) * 0x8);\n");
        }
    }
}

