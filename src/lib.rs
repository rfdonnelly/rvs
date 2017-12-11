extern crate rand;
extern crate linked_hash_map;
extern crate rvs_parser;

pub mod types;

pub use rvs_parser::error::ParseResult;
pub use rvs_parser::error::ParseError;

use types::Context;

pub fn parse(s: &str, context: &mut Context) -> ParseResult<()> {
    let items = rvs_parser::parse(s, &mut context.requires)?;
    context.transform_items(&items);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;
    use std::collections::HashMap;

    mod parse {
        use super::*;

        #[test]
        fn basic() {
            let mut context = Context::new();
            assert!(parse("a=[0,1];\nb=2;", &mut context).is_ok());

            {
                let a = context.get("a").unwrap();
                let result = a.next();
                assert!(result == 0 || result == 1);
            }

            {
                let b = context.get("b").unwrap();
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
            assert!(parse("a=[0,1];b=[2,3];", &mut context).is_ok());
            assert_eq!(context.to_string(), "a = [0x0, 0x1];\nb = [0x2, 0x3];\n");
        }

        #[test]
        fn precendence() {
            let mut context = Context::new();
            assert!(parse("a = (10 + 6) * 8;", &mut context).is_ok());
            assert_eq!(context.to_string(), "a = ((0xa + 0x6) * 0x8);\n");
        }
    }

    mod sample {
        use super::*;

        #[test]
        fn basic() {
            let mut context = Context::new();
            assert!(parse("a = Sample(1, 2, 4, 8);", &mut context).is_ok());

            let a = context.get("a").unwrap();

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
            assert!(parse("a = { 10: 0, 90: 1 };", &mut context).is_ok());

            let a = context.get("a").unwrap();

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

        use std::env::current_dir;

        mod require {
            use super::*;

            /// Verify search path priority
            #[test]
            fn same_filename_different_directory() {
                let mut context = Context::new();
                let fixtures = current_dir().unwrap().join("fixtures/require/same_filename_different_directory");
                context.requires.add_search_path(&fixtures.join("a"));
                context.requires.add_search_path(&fixtures.join("b"));

                parse("require 'a.rvs';", &mut context).unwrap();

                assert!(context.get("a").is_some());
                assert!(context.get("b").is_none());
            }

            #[test]
            fn source_relative() {
                let mut context = Context::new();
                let fixtures = current_dir().unwrap().join("fixtures/require/source_relative");
                context.requires.add_search_path(&fixtures);
                context.requires.add_search_path(&fixtures.join("path"));

                parse("require 'a.rvs';", &mut context).unwrap();

                assert!(context.get("a").is_some());
                assert!(context.get("b").is_some());
            }

            #[test]
            fn require_is_idempotent() {
                let mut context = Context::new();
                let fixtures = current_dir().unwrap().join("fixtures/require/require_is_idempotent");
                context.requires.add_search_path(&fixtures);

                parse("require 'a.rvs';", &mut context).unwrap();

                assert_eq!(context.get("a").unwrap().next(), 2);
            }
        }

        #[test]
        fn readme() {
            let mut context = Context::new();
            context.requires.add_search_path(&::std::env::current_dir().unwrap());
            assert!(parse("require 'examples/readme.rvs';", &mut context).is_ok());

            {
                let pattern = context.get("pattern").unwrap();
                let expected: Vec<u32> = vec![2, 0, 1, 0, 2, 0, 1, 0];
                let mut actual: Vec<u32> = Vec::new();
                for _ in 0..8 {
                    actual.push(pattern.next());
                }
                assert_eq!(expected, actual);
            }

            {
                let sample = context.get("sample").unwrap();
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
                let weighted = context.get("weighted").unwrap();
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

