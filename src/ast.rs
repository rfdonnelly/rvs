#[derive(PartialEq, Debug)]
pub enum Opcode {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(PartialEq, Debug)]
pub enum Node {
    Number(u32),
    Operation(Box<Node>, Opcode, Box<Node>),
}

pub fn eval_walk(node: &Node) -> u32 {
    match *node {
        Node::Number(x) => x,
        Node::Operation(ref bx, ref op, ref by) => {
            let x = eval_walk(bx);
            let y = eval_walk(by);

            match *op {
                Opcode::Add => x + y,
                Opcode::Subtract => x - y,
                Opcode::Multiply => x * y,
                Opcode::Divide => x / y,
            }
        }
    }
}

mod tests {
    mod eval_walk {
        use super::super::*;
        use std::ops::Deref;

        #[test]
        fn add() {
            let expr =
                Box::new(Node::Operation(
                    Box::new(Node::Number(1)),
                    Opcode::Add,
                    Box::new(Node::Number(2))
                ));

            assert_eq!(eval_walk(expr.deref()), 3);
        }

        #[test]
        fn add_mult() {
            let expr =
                Box::new(Node::Operation(
                    Box::new(Node::Number(1)),
                    Opcode::Add,
                    Box::new(Node::Operation(
                        Box::new(Node::Number(2)),
                        Opcode::Multiply,
                        Box::new(Node::Number(3))
                    ))
                ));

            assert_eq!(eval_walk(expr.deref()), 7);
        }
    }
}
