use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Opcode {
    Or,
    Xor,
    And,
    Shl,
    Shr,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(PartialEq, Debug)]
pub enum Node {
    Identifier(String),
    Number(u32),
    Operation(Box<Node>, Opcode, Box<Node>),
    Assignment(Box<Node>, Box<Node>),
    Range(Box<Node>, Box<Node>),
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let operator = match *self {
            Opcode::Or => "|",
            Opcode::Xor => "^",
            Opcode::And => "&",
            Opcode::Shl => "<<",
            Opcode::Shr => ">>",
            Opcode::Add => "+",
            Opcode::Sub => "-",
            Opcode::Mul => "*",
            Opcode::Div => "/",
            Opcode::Mod => "%",
        };

        write!(f, "{}", operator)
    }
}
