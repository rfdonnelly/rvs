use std::fmt;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum UnaryOpcode {
    Neg,
}

#[derive(Debug)]
pub enum Function {
    Pattern,
    Sequence,
    Range,
    Sample,
    WeightedSample,
}

#[derive(Debug)]
pub enum Method {
    Next,
    Prev,
    Copy,
}

#[derive(Debug)]
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
    VariableMethodCall(String, Method),
}

/// An abstraction above Node to implement `import`
#[derive(Debug)]
pub enum Item {
    /// A single item in the current file
    ///
    /// E.g. Node::Assignment
    Single(Box<Node>),

    /// The expansion of a `import` statement
    ///
    /// Contains all items from the `import`d file.
    Multiple(Vec<Item>),

    /// Encapsulates errors on `import`
    ///
    /// We can't use normal Rust error handling techniques due to abstraction by rust-peg.
    /// Instead, embed an Item::ImportErrors on a import error.
    ImportError(::std::path::PathBuf, ::std::io::Error),
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
