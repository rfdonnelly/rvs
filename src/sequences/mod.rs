pub mod value;
pub mod expr;
pub mod range;

use ast::Node;

pub use self::value::Value;
pub use self::expr::Expr;
pub use self::range::RangeSequence;

pub trait Sequence {
    fn next(&mut self) -> u32;
    fn last(&self) -> u32;
}

pub fn sequence_from_ast(node: &Node) -> Box<Sequence> {
    match *node {
        Node::Identifier(_) => panic!("Not supported"),
        Node::Assignment(_, _) => panic!("Not supported"),
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
        }
    }
}

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
}
