use rvs;

pub struct Context {
    parser: rvs::Parser,
    seed: rvs::Seed,
}

impl Context {
    pub fn new(search_path: rvs::SearchPath, seed: rvs::Seed) -> Context {
        Context {
            parser: rvs::Parser::new(&search_path),
            seed,
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
}
