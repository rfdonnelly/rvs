use super::VariableRef;

use indexmap::{map::Entry, IndexMap};
use std::fmt;

#[derive(Default)]
pub struct Model {
    variables: Vec<VariableRef>,
    variable_indexes: IndexMap<String, usize>,
    most_recent: usize,
}

impl Model {
    pub fn new() -> Model {
        Default::default()
    }

    /// Adds a variable to the model
    ///
    /// If the variable `name` already exists in the model, the pre-existing variable is replaced
    /// by the `variable`.
    ///
    /// Returns the index of the variable which saves a call to `get_variable_index`.
    pub fn add_variable(&mut self, name: &str, variable: VariableRef) -> usize {
        let variables = &mut self.variables;
        let most_recent = &mut self.most_recent;

        match self.variable_indexes.entry(name.into()) {
            Entry::Occupied(entry) => {
                *most_recent = *entry.get();
                variables[*most_recent] = variable;

                *most_recent
            }
            Entry::Vacant(entry) => {
                variables.push(variable);
                *most_recent = variables.len() - 1;
                entry.insert(*most_recent);

                *most_recent
            }
        }
    }

    pub fn get_variable_index(&self, name: &str) -> Option<usize> {
        let index = self.variable_indexes.get(name)?;
        Some(*index)
    }

    pub fn get_variable_by_index(&self, index: usize) -> Option<&VariableRef> {
        let variable = self.variables.get(index)?;
        Some(variable)
    }

    pub fn get_variable_by_name(&self, name: &str) -> Option<&VariableRef> {
        let index = self.variable_indexes.get(name)?;
        let variable = self.variables.get(*index)?;
        Some(variable)
    }

    pub fn get_most_recently_added(&self) -> Option<&VariableRef> {
        self.variables.get(self.most_recent)
    }

    pub fn variables_iter(&self) -> VariablesIter {
        VariablesIter {
            iter: self.variable_indexes.iter(),
            variables: &self.variables,
        }
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (name, variable) in self.variables_iter() {
            write!(f, "{} = ", name)?;
            variable.borrow().fmt(f)?;
            writeln!(f, ";")?;
        }

        Ok(())
    }
}

pub struct VariablesIter<'a> {
    iter: ::indexmap::map::Iter<'a, String, usize>,
    variables: &'a Vec<VariableRef>,
}

impl<'a> Iterator for VariablesIter<'a> {
    type Item = (&'a str, &'a VariableRef);

    fn next(&mut self) -> Option<(&'a str, &'a VariableRef)> {
        let next = self.iter.next()?;

        Some((next.0, &self.variables[*next.1]))
    }
}
