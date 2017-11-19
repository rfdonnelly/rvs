pub mod value;
pub mod expr;
pub mod range;

use std::fmt;
use std::ops::Deref;
use linked_hash_map::LinkedHashMap;
use rand::Rng;
use rand::SeedableRng;
use rand::prng::XorShiftRng;
use rand::Sample;

use ast::Node;
use ast::Function;
use ast::Item;

pub use self::value::Value;
pub use self::expr::Expr;
pub use self::range::RangeSequence;

pub struct RvData {
    prev: u32,
    done: bool,
}

pub trait Rv: fmt::Display {
    fn next(&mut self, rng: &mut Rng) -> u32;

    fn prev(&self) -> u32 {
        self.data().prev
    }

    fn done(&self) -> bool {
        self.data().done
    }

    fn data(&self) -> &RvData;
}

/// Random Variable Container
pub struct RvC {
    root: Box<Rv>,
    rng: Box<Rng>,
}

impl RvC {
    pub fn next(&mut self) -> u32 {
        self.root.next(&mut self.rng)
    }

    pub fn prev(&self) -> u32 {
        self.root.prev()
    }

    pub fn done(&self) -> bool {
        self.root.done()
    }
}

impl fmt::Display for RvC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.root.fmt(f)
    }
}

pub struct Seed {
    pub value: [u32; 4],
}

impl Seed {
    /// Generates a 128-bit seed from a 32-bit seed
    ///
    /// This is done via two steps:
    ///
    /// 1. Create a low quality 128-bit seed (LQS)
    ///
    ///    This is done with simple bit manipulation of the 32-bit seed.
    ///
    /// 2. Create a higher quality 128-bit seed (HQS)
    ///
    ///    This is done by seeding an Rng with the LQS then using the Rng to generate the HQS.
    pub fn from_u32(seed: u32) -> Seed {
        let mut rng = XorShiftRng::from_seed([
            seed,
            seed ^ 0xaaaa_aaaa,
            seed ^ 0x5555_5555,
            !seed,
        ]);

        Seed {
            value: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
        }
    }
}

pub struct Context {
    pub variables: Vec<Box<RvC>>,
    pub handles: LinkedHashMap<String, usize>,
    pub enums: LinkedHashMap<String, Enum>,
    pub seed: Seed,
}

impl Context {
    pub fn new() -> Context {
        Context {
            variables: Vec::new(),
            handles: LinkedHashMap::new(),
            enums: LinkedHashMap::new(),
            seed: Seed::from_u32(0),
        }
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (id, handle) in self.handles.iter() {
            write!(f, "{} = ", id)?;
            self.variables[*handle].fmt(f)?;
            writeln!(f, ";")?;
        }

        Ok(())
    }
}

pub struct Enum {
    name: String,
    items: LinkedHashMap<String, u32>,
}

impl Enum {
    pub fn new(name: String, items: LinkedHashMap<String, u32>) -> Enum {
        Enum {
            name,
            items,
        }
    }
}

impl Context {
    pub fn rvs_from_ast(&mut self, items: &Vec<Item>) {
        for item in items {
            match *item {
                Item::Single(ref node) => {
                    match *node.deref() {
                        Node::Assignment(ref lhs, ref rhs) => {
                            let mut identifier: String = "".into();

                            if let Node::Identifier(ref x) = **lhs {
                                identifier = x.clone();
                            }

                            let mut rng = new_rng(&self.seed);
                            let rvc = Box::new(RvC {
                                root: self.rv_from_ast(&mut rng, &rhs),
                                rng: rng,
                            });
                            self.variables.push(rvc);
                            self.handles.insert(identifier, self.variables.len() - 1);
                        },
                        Node::Enum(ref name, ref enum_items_vec) => {
                            let mut enum_items_map = LinkedHashMap::new();

                            // FIXME Convert to .map()
                            for item in enum_items_vec.iter() {
                                if let Node::EnumItem(ref name, ref value_node) = *item.deref() {
                                    if let Node::Number(value) = *value_node.deref() {
                                        // FIXME Check for existence
                                        enum_items_map.insert(name.to_owned(), value);
                                    } else {
                                        panic!("Expected Number but found...FIXME");
                                    }
                                } else {
                                    panic!("Expected EnumItem but found...FIXME");
                                }
                            }
                            self.enums.insert(
                                name.to_owned(),
                                Enum::new(name.to_owned(), enum_items_map)
                            );
                        }
                        _ => {},
                    }
                }
                Item::Multiple(ref items) => {
                    self.rvs_from_ast(items)
                }
            }
        }
    }

    pub fn rv_from_ast(&self, rng: &mut Rng, node: &Node) -> Box<Rv> {
        match *node {
            Node::Function(ref function, ref args) => {
                match *function {
                    Function::Range => {
                        let l = self.rv_from_ast(rng, &args[0]).next(rng);
                        let r = self.rv_from_ast(rng, &args[1]).next(rng);

                        Box::new(
                            RangeSequence::new(l, r)
                        )
                    }
                }
            }
            Node::Number(x) => Box::new(Value::new(x)),
            Node::Operation(ref bx, ref op, ref by) => {
                Box::new(
                    Expr::new(
                        self.rv_from_ast(rng, bx),
                        op.clone(),
                        self.rv_from_ast(rng, by)
                    )
                )
            },
            Node::EnumItemInst(ref a, ref b) => {
                if let Some(entry) = self.enums.get(a) {
                    if let Some(entry) = entry.items.get(b) {
                        Box::new(
                            Value::new(*entry)
                        )
                    } else {
                        panic!("Could not find enum value '{}' in enum '{}'", b, a);
                    }
                } else {
                    panic!("Could not find enum '{}'", a);
                }
            },
            _ => panic!("Not supported"),
        }
    }
}

fn new_rng(seed: &Seed) -> Box<Rng> {
    Box::new(XorShiftRng::from_seed(seed.value))
}


#[cfg(test)]
mod tests {
    use super::*;

    use linked_hash_map::Entry::Occupied;

    mod rv_from_ast {
        use super::*;

        use std::collections::HashMap;

        #[test]
        fn number() {
            let context = Context::new();
            let mut rng = new_rng(&Seed::from_u32(0));
            let ast = Node::Number(4);
            let mut variable = context.rv_from_ast(&mut rng, &ast);

            assert_eq!(variable.next(&mut rng), 4);
        }

        #[test]
        fn range() {
            let context = Context::new();
            let mut rng = new_rng(&Seed::from_u32(0));
            let ast = Node::Function(
                Function::Range,
                vec![
                    Box::new(Node::Number(3)),
                    Box::new(Node::Number(4))
                ]
            );
            let mut variable = context.rv_from_ast(&mut rng, &ast);

            let mut values = HashMap::new();

            for _ in 0..10 {
                let value = variable.next(&mut rng);
                let entry = values.entry(value).or_insert(0);
                *entry += 1;
                assert!(value == 3 || value == 4);
            }

            assert!(values[&3] > 0);
            assert!(values[&4] > 0);
        }
    }

    mod rvs_from_ast {
        use super::*;

        #[test]
        fn basic() {
            let items = vec![
                Item::Single(
                    Box::new(Node::Assignment(
                        Box::new(Node::Identifier("a".into())),
                        Box::new(Node::Number(5))
                    ))
                ),
                Item::Single(
                    Box::new(Node::Assignment(
                        Box::new(Node::Identifier("b".into())),
                        Box::new(Node::Number(6))
                    ))
                ),
            ];

            let mut context = Context::new();
            context.rvs_from_ast(&items);

            assert!(context.handles.contains_key("a"));
            if let Occupied(entry) = context.handles.entry("a".into()) {
                let id = entry.get();
                let value = context.variables[*id].next();
                assert_eq!(value, 5);
            }
            assert!(context.handles.contains_key("b"));
            if let Occupied(entry) = context.handles.entry("b".into()) {
                let id = entry.get();
                let value = context.variables[*id].next();
                assert_eq!(value, 6);
            }
        }
    }
}
