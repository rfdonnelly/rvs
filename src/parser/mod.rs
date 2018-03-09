mod ast;
#[cfg_attr(feature = "cargo-clippy", allow(module_inception))]
mod parser;

pub use self::parser::Parser;
