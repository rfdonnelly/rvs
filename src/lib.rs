extern crate rand;
extern crate linked_hash_map;

pub mod ast;
pub mod grammar;
pub mod types;
pub mod error;

use types::Context;
use error::ParseError;
use error::ParseResult;

pub fn parse_rvs(s: &str, context: &mut Context) -> ParseResult<()> {
    match grammar::items(s) {
        Ok(items) => {
            context.transform_items(&items);

            Ok(())
        },
        Err(error) => {
            // FIXME: Improve formatting source code in errors
            //
            // Current format:
            //
            // error at 2:3: expected `=`
            // a += b;
            //   ^
            //
            // Example: rustc
            //
            // error: expected expression, found `+`
            //   --> /home/rfdonnelly/repos/rvs/src/lib.rs:28:24
            //    |
            // 28 |                 error, +
            //    |                        ^
            //
            // Notable features:
            //
            // * Source file path
            // * Single space above and below source line
            // * Source line prefixed with line number and '|' separator
            let mut indent = String::with_capacity(error.column);
            for _ in 0..error.column - 1 {
                indent.push_str(" ");
            }
            let line = s.lines().nth(error.line - 1).unwrap();
            let description = format!(
                "{}\n{}\n{}^",
                error,
                line,
                indent,
            );
            Err(ParseError::new(description))
        }
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

    mod examples {
        use super::*;

        #[test]
        fn readme() {
            let mut context = Context::new();
            assert!(parse_rvs("require 'examples/readme.rvs';", &mut context).is_ok());

            {
                let pattern = context.get_variable("pattern").unwrap();
                let expected: Vec<u32> = vec![2, 0, 1, 0, 2, 0, 1, 0];
                let mut actual: Vec<u32> = Vec::new();
                for _ in 0..8 {
                    actual.push(pattern.next());
                }
                assert_eq!(expected, actual);
            }

            {
                let sample = context.get_variable("sample").unwrap();
                let mut results: HashMap<u32, u32> = HashMap::new();
                for _ in 0..90 {
                    let result = sample.next();
                    let entry = results.entry(result).or_insert(0);
                    *entry += 1;;
                }
                assert_eq!(results.len(), 3);
                for i in 0..3 {
                    assert!(results[&i] >= 30 - 5 && results[&i] <= 30 + 5);
                }
            }

            {
                let weighted = context.get_variable("weighted").unwrap();
                let mut results: HashMap<u32, u32> = HashMap::new();
                for _ in 0..1000 {
                    let result = weighted.next();
                    let entry = results.entry(result).or_insert(0);
                    *entry += 1;;
                }
                println!("weighted: {:?}", results);
                assert_eq!(results.len(), 3);
                assert!(results[&0] >= 400 - 50 && results[&0] <= 400 + 50);
                assert!(results[&1] >= 500 - 50 && results[&1] <= 500 + 50);
                assert!(results[&2] >= 100 - 50 && results[&2] <= 100 + 50);
            }
        }
    }
}

