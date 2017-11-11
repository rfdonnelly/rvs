pub mod value;
pub mod expr;
pub mod range;

use rand::Rng;
use rand::SeedableRng;
use rand::prng::ChaChaRng;
use std::collections::HashMap;

use ast::Node;

pub use self::value::Value;
pub use self::expr::Expr;
pub use self::range::RangeSequence;

pub struct RvData {
    prev: u32,
    done: bool,
}

pub trait Rv {
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

pub fn rvs_from_ast(assignments: Vec<Box<Node>>, ids: &mut HashMap<String, usize>, variables: &mut Vec<Box<RvC>>) {
    for assignment in assignments {
        if let Node::Assignment(ref lhs, ref rhs) = *assignment {
            let mut identifier: String = "".into();

            if let Node::Identifier(ref x) = **lhs {
                identifier = x.clone();
            }

            // FIXME: Allow non-const seed
            let mut rng = new_rng();
            variables.push(Box::new(RvC {
                root: rv_from_ast(&mut rng, &rhs),
                rng: rng,
            }));
            ids.insert(identifier, variables.len() - 1);
        }
    }
}

fn new_rng() -> Box<Rng> {
    Box::new(ChaChaRng::from_seed(&[0x0000_0000]))
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
    mod rv_from_ast {
        use super::super::*;

        #[test]
        fn number() {
            let mut rng = new_rng();
            let ast = Node::Number(4);
            let mut variable = rv_from_ast(&mut rng, &ast);

            assert_eq!(variable.next(&mut rng), 4);
        }

        #[test]
        fn range() {
            use std::collections::HashMap;

            let mut rng = new_rng();
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
        use super::super::*;

        use std::collections::hash_map::Entry::Occupied;

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

            let mut ids = HashMap::new();
            let mut variables = Vec::new();
            rvs_from_ast(assignments, &mut ids, &mut variables);

            assert!(ids.contains_key("a"));
            if let Occupied(entry) = ids.entry("a".into()) {
                let id = entry.get();
                let value = variables[*id].next();
                assert_eq!(value, 5);
            }
            assert!(ids.contains_key("b"));
            if let Occupied(entry) = ids.entry("b".into()) {
                let id = entry.get();
                let value = variables[*id].next();
                assert_eq!(value, 6);
            }
        }
    }
}
