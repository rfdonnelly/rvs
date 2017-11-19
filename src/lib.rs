extern crate rand;
extern crate libc;
extern crate linked_hash_map;

mod ast;
mod grammar;
mod types;
pub mod c_api;

use types::Context;
use grammar::ParseResult;

fn parse_rvs(s: &str, context: &mut Context) -> ParseResult<()> {
    match grammar::items(s) {
        Ok(items) => {
            context.transform_items(&items);

            Ok(())
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;
    use std::collections::HashMap;

    mod parse_rvs {
        use super::*;

        #[test]
        fn basic() {
            let mut context = Context::new();
            assert!(parse_rvs("a=[0,1];\nb=2;", &mut context).is_ok());

            {
                let a = context.get_variable("a").unwrap();
                let result = a.next();
                assert!(result == 0 || result == 1);
            }

            {
                let b = context.get_variable("b").unwrap();
                let result = b.next();
                assert_eq!(result, 2);
            }
        }
    }

    mod display {
        use super::*;

        #[test]
        fn multiple() {
            let mut context = Context::new();
            assert!(parse_rvs("a=[0,1];b=[2,3];", &mut context).is_ok());
            assert_eq!(context.to_string(), "a = [0x0, 0x1];\nb = [0x2, 0x3];\n");
        }

        #[test]
        fn precendence() {
            let mut context = Context::new();
            assert!(parse_rvs("a = (10 + 6) * 8;", &mut context).is_ok());
            assert_eq!(context.to_string(), "a = ((0xa + 0x6) * 0x8);\n");
        }
    }

    mod sample {
        use super::*;

        #[test]
        fn basic() {
            let mut context = Context::new();
            assert!(parse_rvs("a = Sample(1, 2, 4, 8);", &mut context).is_ok());

            let a = context.get_variable("a").unwrap();

            let expected: HashSet<u32> =
                [1, 2, 4, 8].iter().cloned().collect();
            let mut actual: HashSet<u32> = HashSet::new();

            for _ in 0..16 {
                actual.insert(a.next());
            }

            assert_eq!(expected, actual);
        }
    }

    mod weighted_sample {
        use super::*;

        #[test]
        fn basic() {
            let mut context = Context::new();
            assert!(parse_rvs("a = { 10: 0, 90: 1 };", &mut context).is_ok());

            let a = context.get_variable("a").unwrap();

            let mut results: HashMap<u32, u32> = HashMap::new();

            for _ in 0..1000 {
                let result = a.next();
                let entry = results.entry(result).or_insert(0);
                *entry += 1;
            }

            assert!(results[&0] >= 100 - 10 && results[&0] <= 100 + 10);
            assert!(results[&1] >= 900 - 10 && results[&1] <= 900 + 10);
        }
    }
}

