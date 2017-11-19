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
pub enum Function {
    Range,
}

#[derive(PartialEq, Debug)]
pub enum Node {
    Identifier(String),
    Number(u32),
    Operation(Box<Node>, Opcode, Box<Node>),
    Assignment(Box<Node>, Box<Node>),
    Enum(String, Vec<Box<Node>>),
    EnumItem(String, Box<Node>),
    EnumItemInst(String, String),
    Function(Function, Vec<Box<Node>>),
}

/// An abstraction above Node to implement `require`
#[derive(PartialEq, Debug)]
pub enum Item {
    /// A single item in the current file
    ///
    /// E.g. Node::Assignment
    Single(Box<Node>),

    /// The expansion of a `require` statement
    ///
    /// Contains all items from the `require`d file.
    Multiple(Vec<Item>),
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
