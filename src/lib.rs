extern crate rand;
extern crate linked_hash_map;
extern crate rvs_parser;

pub mod types;
pub mod error;

pub use rvs_parser::error::ParseError;
pub use error::TransformError;
pub use error::Error;

use types::Context;

pub fn parse(s: &str, context: &mut Context) -> Result<(), Error> {
    let items = rvs_parser::parse(s, &mut context.imports)?;
    context.transform_items(&items).unwrap();
    Ok(())
}
