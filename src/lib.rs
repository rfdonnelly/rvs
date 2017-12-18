extern crate rand;
extern crate linked_hash_map;
extern crate rvs_parser;

pub mod types;
pub mod error;

pub use rvs_parser::error::ParseResult;
pub use rvs_parser::error::ParseError;

use types::Context;

pub fn parse(s: &str, context: &mut Context) -> ParseResult<()> {
    let items = rvs_parser::parse(s, &mut context.requires)?;
    context.transform_items(&items).unwrap();
    Ok(())
}
