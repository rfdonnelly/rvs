extern crate rand;
extern crate linked_hash_map;
extern crate rvs_parser;

mod error;

pub mod types;

pub use types::{
    Context,
    Seed,
};

pub use error::{
    Error,
    Result,
};

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
