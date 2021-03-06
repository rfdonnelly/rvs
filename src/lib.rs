mod error;
mod parser;
mod transform;
mod model;
mod types;

pub use rvs_parser::SearchPath;
pub use crate::parser::Parser;
pub use crate::transform::{Seed, Transform};
pub use crate::model::{Model, Variable};

pub use crate::error::{Error, Result};

pub fn parse(search_path: &SearchPath, s: &str) -> Result<Model> {
    let mut parser = Parser::new(search_path);
    parser.parse(s)?;

    let mut transform = Transform::new(Default::default());
    let mut model = Model::new();
    transform.transform(&mut model, parser.ast())?;
    Ok(model)
}
