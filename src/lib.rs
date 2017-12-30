extern crate rand;
extern crate linked_hash_map;
extern crate rvs_parser;

pub mod types;
pub mod error;

pub use rvs_parser::error::ParseError;
pub use error::TransformError;
pub use error::Error;
pub use error::Result;

use types::Context;

pub fn parse(s: &str, context: &mut Context) -> Result<()> {
    let items = rvs_parser::parse(s, &mut context.search_path)?;
    context.transform_items(&items).unwrap();
    Ok(())
}
