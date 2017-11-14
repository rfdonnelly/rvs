pub mod value;
pub mod expr;
pub mod range;

use std::fmt;
use linked_hash_map::LinkedHashMap;
use rand::Rng;
use rand::SeedableRng;
use rand::prng::XorShiftRng;
use rand::Sample;

use ast::Node;

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
    pub seed: Seed,
}

impl Context {
    pub fn new() -> Context {
        Context {
            variables: Vec::new(),
            handles: LinkedHashMap::new(),
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

pub fn rvs_from_ast(assignments: Vec<Box<Node>>, context: &mut Context) {
    for assignment in assignments {
        if let Node::Assignment(ref lhs, ref rhs) = *assignment {
            let mut identifier: String = "".into();

            if let Node::Identifier(ref x) = **lhs {
                identifier = x.clone();
            }

            let mut rng = new_rng(&context.seed);
            context.variables.push(Box::new(RvC {
                root: rv_from_ast(&mut rng, &rhs),
                rng: rng,
            }));
            context.handles.insert(identifier, context.variables.len() - 1);
        }
    }
}

fn new_rng(seed: &Seed) -> Box<Rng> {
    Box::new(XorShiftRng::from_seed(seed.value))
}

pub fn rv_from_ast(rng: &mut Rng, node: &Node) -> Box<Rv> {
    match *node {
        Node::Range(ref bx, ref by) => {
            let l = rv_from_ast(rng, bx).next(rng);
            let r = rv_from_ast(rng, by).next(rng);

            Box::new(
                RangeSequence::new(l, r)
            )
        }
        Node::Number(x) => Box::new(Value::new(x)),
        Node::Operation(ref bx, ref op, ref by) => {
            Box::new(
                Expr::new(
                    rv_from_ast(rng, bx),
                    op.clone(),
                    rv_from_ast(rng, by)
                )
            )
        },
        _ => panic!("Not supported"),
    }
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
            let mut rng = new_rng(&Seed::from_u32(0));
            let ast = Node::Number(4);
            let mut variable = rv_from_ast(&mut rng, &ast);

            assert_eq!(variable.next(&mut rng), 4);
        }

        #[test]
        fn range() {
            let mut rng = new_rng(&Seed::from_u32(0));
            let ast = Node::Range(
                Box::new(Node::Number(3)),
                Box::new(Node::Number(4))
            );
            let mut variable = rv_from_ast(&mut rng, &ast);

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
            let assignments = vec![
                Box::new(Node::Assignment(
                    Box::new(Node::Identifier("a".into())),
                    Box::new(Node::Number(5))
                )),
                Box::new(Node::Assignment(
                    Box::new(Node::Identifier("b".into())),
                    Box::new(Node::Number(6))
                )),
            ];

            let mut context = Context::new();
            rvs_from_ast(assignments, &mut context);

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
