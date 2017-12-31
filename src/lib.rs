extern crate rand;
extern crate linked_hash_map;
extern crate rvs_parser;

pub mod types;
pub mod error;

pub use types::Context;
pub use types::Seed;

pub use error::Error;
pub use error::Result;

pub fn parse(s: &str, context: &mut Context) -> Result<()> {
    let parser = rvs_parser::Parser::new(context.search_path.clone());
    let items = parser.parse(s)?;
    context.transform_items(items)?;
    Ok(())
}

pub fn transform(context: &mut Context) -> Result<()> {
    context.transform_variables()?;
    Ok(())
}
