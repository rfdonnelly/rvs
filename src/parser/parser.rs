use super::ast::Ast;

use error::Result;

use rvs_parser;
use rvs_parser::ast;

pub struct Parser {
    search_path: rvs_parser::SearchPath,
    ast: Ast,
}

impl Parser {
    pub fn new(
        search_path: &rvs_parser::SearchPath
    ) -> Parser {
        Parser {
            // FIXME: Remove clone
            search_path: search_path.clone(),
            ast: Ast::new(),
        }
    }

    pub fn parse(
        &mut self,
        s: &str
    ) -> Result<()> {
        // FIXME: Remove clone
        let parser = rvs_parser::Parser::new(self.search_path.clone());
        let nodes = parser.parse(s)?;
        self.ast.add_nodes(nodes);

        Ok(())
    }

    pub fn ast(&self) -> &[Box<ast::Node>] {
        self.ast.get()
    }
}
