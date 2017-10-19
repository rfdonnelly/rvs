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
