pub mod ast;
mod grammar;
pub mod path;
pub mod error;

pub use self::path::RequirePaths;
pub use error::ParseResult;
pub use error::ParseError;

pub fn parse(s: &str, require_paths: &mut RequirePaths) -> ParseResult<Vec<Box<ast::Node>>> {
    match grammar::items(s, require_paths) {
        Ok(items) => Ok(flatten(items)),
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
            let description = format!(
                "{}\n{}\n{}^",
                error,
                line,
                indent,
            );

            Err(ParseError::new(description))
        }
    }
}

fn flatten_recursive(mut items: Vec<ast::Item>, nodes: &mut Vec<Box<ast::Node>>) {
    for item in items.drain(..) {
        match item {
            ast::Item::Single(node) => nodes.push(node),
            ast::Item::Multiple(items) => flatten_recursive(items, nodes),
        }
    }
}

/// Strips out all ast::Items while keeping their contents
///
/// ast::Items only serve as packaging for ast::Nodes.  `require` adds the packaging.  `flatten`
/// removes the packaging.  ast::Items are an implementation detail for `require` and only add
/// noise to the AST.
fn flatten(items: Vec<ast::Item>) -> Vec<Box<ast::Node>> {
    let mut nodes: Vec<Box<ast::Node>> = Vec::new();

    flatten_recursive(items, &mut nodes);

    nodes
}
