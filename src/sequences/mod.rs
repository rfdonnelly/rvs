pub mod value;
pub mod expr;
pub mod range;

use std::collections::HashMap;

use ast::Node;

pub use self::value::Value;
pub use self::expr::Expr;
pub use self::range::RangeSequence;

pub trait Sequence : Send + Sync {
    fn next(&mut self) -> u32;
    fn last(&self) -> u32;
}

pub fn sequences_from_ast(assignments: Vec<Box<Node>>, ids: &mut HashMap<String, usize>, sequences: &mut Vec<Box<Sequence>>) {
    for assignment in assignments {
        if let Node::Assignment(ref lhs, ref rhs) = *assignment {
            let mut identifier: String = "".into();

            if let Node::Identifier(ref x) = **lhs {
                identifier = x.clone();
            }

            sequences.push(sequence_from_ast(&rhs));
            ids.insert(identifier, sequences.len() - 1);
        }
    }
}

pub fn sequence_from_ast(node: &Node) -> Box<Sequence> {
    match *node {
        Node::Range(ref bx, ref by) => {
            Box::new(
                RangeSequence::new(
                    &mut *sequence_from_ast(bx),
                    &mut *sequence_from_ast(by)
                )
            )
        }
        Node::Number(x) => Box::new(Value::new(x)),
        Node::Operation(ref bx, ref op, ref by) => {
            Box::new(
                Expr::new(
                    sequence_from_ast(bx),
                    op.clone(),
                    sequence_from_ast(by)
                )
            )
        },
        _ => panic!("Not supported"),
    }
}

#[cfg(test)]
mod tests {
    mod sequence_from_ast {
        use super::super::*;

        #[test]
        fn number() {
            let ast = Node::Number(4);
            let mut sequence = sequence_from_ast(&ast);

            assert_eq!(sequence.next(), 4);
        }

        #[test]
        fn range() {
            use std::collections::HashMap;

            let ast = Node::Range(
                Box::new(Node::Number(3)),
                Box::new(Node::Number(4))
            );
            let mut sequence = sequence_from_ast(&ast);

            let mut values = HashMap::new();

            for _ in 0..10 {
                let value = sequence.next();
                let entry = values.entry(value).or_insert(0);
                *entry += 1;
                assert!(value == 3 || value == 4);
            }

            assert!(values[&3] > 0);
            assert!(values[&4] > 0);
        }
    }

    mod sequences_from_ast {
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
            let mut sequences = Vec::new();
            sequences_from_ast(assignments, &mut ids, &mut sequences);

            assert!(ids.contains_key("a"));
            if let Occupied(entry) = ids.entry("a".into()) {
                let id = entry.get();
                let value = sequences[*id].next();
                assert_eq!(value, 5);
            }
            assert!(ids.contains_key("b"));
            if let Occupied(entry) = ids.entry("b".into()) {
                let id = entry.get();
                let value = sequences[*id].next();
                assert_eq!(value, 6);
            }
        }
    }
}
