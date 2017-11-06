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
