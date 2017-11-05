#[derive(PartialEq, Debug, Clone)]
pub enum Opcode {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(PartialEq, Debug)]
pub enum Node {
    Identifier(String),
    Number(u32),
    Operation(Box<Node>, Opcode, Box<Node>),
    Assignment(Box<Node>, Box<Node>),
    Range(Box<Node>, Box<Node>),
}

impl Node {
    pub fn eval(&self) -> u32 {
        match *self {
            Node::Number(x) => x,
            Node::Operation(ref bx, ref op, ref by) => {
                let x = bx.eval();
                let y = by.eval();

                match *op {
                    Opcode::Add => x + y,
                    Opcode::Subtract => x - y,
                    Opcode::Multiply => x * y,
                    Opcode::Divide => x / y,
                    Opcode::Modulo => x % y,
                }
            }
            _ => panic!("Not supported in this context"),
        }
    }
}

#[cfg(test)]
mod tests {
    mod eval {
        use super::super::*;

        #[test]
        fn add() {
            let expr =
                Box::new(Node::Operation(
                    Box::new(Node::Number(1)),
                    Opcode::Add,
                    Box::new(Node::Number(2))
                ));

            assert_eq!(expr.eval(), 3);
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

            assert_eq!(expr.eval(), 7);
        }
    }
}
