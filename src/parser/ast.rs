use rvs_parser::ast;

use std::collections::HashMap;
use std::collections::hash_map::Entry;

/// Collection of AST nodes
///
/// Takes one or more ASTs as input and outputs a minimized AST.
///
/// The AST is minimized by discarding all but the last variable defintion for each variable.
pub struct Ast {
    // Indexes to variable nodes in the nodes Vec
    variable_indexes: HashMap<String, usize>,
    nodes: Vec<Box<ast::Node>>,
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            variable_indexes: HashMap::new(),
            nodes: Vec::new(),
        }
    }

    /// Adds AST nodes to the AST.
    ///
    /// * If Node is a variable definition
    ///
    ///   * If variable has been previously defined
    ///
    ///     Lookup index and replace previous definition at index with new definition
    ///
    ///   * Else, push node into the AST
    ///
    /// * Else, push node into the AST
    pub fn add_nodes(&mut self, mut nodes: Vec<Box<ast::Node>>) {
        for node in nodes.drain(..) {
            self.add_node(node);
        }
    }

    fn add_node(&mut self, node: Box<ast::Node>) {
        let is_variable = if let ast::Node::Variable(_, _) = *node {
            true
        } else {
            false
        };

        if is_variable {
            let name = if let ast::Node::Variable(ref name, _) = *node {
                name.to_owned()
            } else {
                "".to_owned()
            };

            let nodes = &mut self.nodes;
            match self.variable_indexes.entry(name) {
                Entry::Occupied(entry) => {
                    let index = *entry.get();
                    *nodes.get_mut(index).unwrap() = node;
                }
                Entry::Vacant(entry) => {
                    nodes.push(node);
                    entry.insert(nodes.len() - 1);
                }
            }
        } else {
            self.nodes.push(node);
        }
    }

    pub fn get(&self) -> &[Box<ast::Node>] {
        &self.nodes
    }
}
