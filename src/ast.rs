use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum BinaryOpcode {
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

#[derive(PartialEq, Debug, Clone)]
pub enum UnaryOpcode {
    Neg,
}

#[derive(PartialEq, Debug)]
pub enum Function {
    Pattern,
    Range,
    Sample,
    WeightedSample,
}

#[derive(PartialEq, Debug)]
pub enum Node {
    Identifier(String),
    Number(u32),
    UnaryOperation(UnaryOpcode, Box<Node>),
    BinaryOperation(Box<Node>, BinaryOpcode, Box<Node>),
    Assignment(Box<Node>, Box<Node>),
    Enum(String, Vec<Box<Node>>),
    EnumItem(String, Option<Box<Node>>),
    EnumInst(String),
    EnumItemInst(String, String),
    Function(Function, Vec<Box<Node>>),
    WeightedPair(u32, Box<Node>),
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

impl fmt::Display for BinaryOpcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let operator = match *self {
            BinaryOpcode::Or => "|",
            BinaryOpcode::Xor => "^",
            BinaryOpcode::And => "&",
            BinaryOpcode::Shl => "<<",
            BinaryOpcode::Shr => ">>",
            BinaryOpcode::Add => "+",
            BinaryOpcode::Sub => "-",
            BinaryOpcode::Mul => "*",
            BinaryOpcode::Div => "/",
            BinaryOpcode::Mod => "%",
        };

        write!(f, "{}", operator)
    }
}

impl fmt::Display for UnaryOpcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let operator = match *self {
            UnaryOpcode::Neg => "~",
        };

        write!(f, "{}", operator)
    }
}
