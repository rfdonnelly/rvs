use ast;
use grammar;
use searchpath::SearchPath;
use sourcepaths::SourcePaths;
use error::{Error, ParseError, Result};

pub struct Parser {
    searchpath: SearchPath,
}

impl Parser {
    pub fn new(searchpath: SearchPath) -> Parser {
        Parser { searchpath }
    }

    pub fn parse(&self, s: &str) -> Result<Vec<Box<ast::Node>>> {
        match grammar::items(s, &mut SourcePaths::new(self.searchpath.clone())) {
            Ok(items) => self.flatten(items),
            Err(error) => {
                // FIXME: Improve formatting source code in errors
                //
                // Current format:
                //
                // error at 2:3: expected `=`
                // a += b;
                //   ^
                //
                // Example: rustc
                //
                // error: expected expression, found `+`
                //   --> /home/rfdonnelly/repos/rvs/src/lib.rs:28:24
                //    |
                // 28 |                 error, +
                //    |                        ^
                //
                // Notable features:
                //
                // * Source file path
                // * Single space above and below source line
                // * Source line prefixed with line number and '|' separator
                let mut indent = String::with_capacity(error.column);
                for _ in 0..error.column - 1 {
                    indent.push_str(" ");
                }
                let line = s.lines().nth(error.line - 1).unwrap();
                let description = format!("{}\n{}\n{}^", error, line, indent,);

                Err(Error::Parse(ParseError::new(description)))
            }
        }
    }

    fn flatten_recursive(
        &self,
        mut items: Vec<ast::Item>,
        nodes: &mut Vec<Box<ast::Node>>,
    ) -> Result<()> {
        for item in items.drain(..) {
            match item {
                ast::Item::Single(node) => nodes.push(node),
                ast::Item::Multiple(items) => self.flatten_recursive(items, nodes)?,
                ast::Item::ImportError(path, err) => {
                    return Err(Error::Io(err));
                }
            }
        }

        Ok(())
    }

    /// Strips out all ast::Items while keeping their contents
    ///
    /// ast::Items only serve as packaging for ast::Nodes.  `import` adds the packaging.  `flatten`
    /// removes the packaging.  ast::Items are an implementation detail for `import` and only add
    /// noise to the AST.
    fn flatten(&self, items: Vec<ast::Item>) -> Result<Vec<Box<ast::Node>>> {
        let mut nodes: Vec<Box<ast::Node>> = Vec::new();

        self.flatten_recursive(items, &mut nodes)?;

        Ok(nodes)
    }
}
