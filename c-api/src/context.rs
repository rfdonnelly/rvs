use rvs;

use std::io;
use std::path::Path;
use std::path::PathBuf;

pub struct Context {
    parser: rvs::Parser,
    seed: rvs::Seed,
    search_path: rvs::SearchPath,
}

impl Context {
    pub fn new(search_path: rvs::SearchPath, seed: rvs::Seed) -> Context {
        Context {
            parser: rvs::Parser::new(&search_path),
            seed,
            search_path,
        }
    }

    pub fn parse(&mut self, s: &str) -> rvs::Result<()> {
        self.parser.parse(s)
    }

    pub fn transform(&self, model: &mut rvs::Model) -> rvs::Result<()> {
        let mut transform = rvs::Transform::new(self.seed.clone());

        transform.transform(model, self.parser.ast())?;

        Ok(())
    }

    pub fn find_file(&self, path: &Path) -> io::Result<PathBuf> {
        self.search_path.find(path)
    }
}
