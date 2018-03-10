mod grammar {
    #![cfg_attr(feature = "cargo-clippy", allow(clippy))]
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}
mod searchpath;
mod sourcepaths;
mod parser;

pub mod ast;
pub mod error;

pub use searchpath::SearchPath;
pub use parser::Parser;
pub use error::Error;
pub use error::ParseError;
