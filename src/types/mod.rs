pub mod value;
pub mod operation;
pub mod pattern;
pub mod range;
pub mod sample;
pub mod weightedsample;

use std::fmt;
use linked_hash_map::LinkedHashMap;
use rand::Rng;
use rand::SeedableRng;
use rand::prng::XorShiftRng;
use rand::Sample as RandSample;

use rvs_parser::ast;
use rvs_parser::RequirePaths;

pub use self::value::Value;
pub use self::operation::Unary;
pub use self::operation::Binary;
pub use self::pattern::Pattern;
pub use self::range::RangeSequence;
pub use self::sample::Sample;
pub use self::weightedsample::WeightedSample;

pub struct ExprData {
    prev: u32,
    done: bool,
}

pub trait Expr: fmt::Display {
    fn next(&mut self, rng: &mut Rng) -> u32;

    fn prev(&self) -> u32 {
        self.data().prev
    }

    fn done(&self) -> bool {
        self.data().done
    }

    fn data(&self) -> &ExprData;
}

/// Random Variable
pub struct Rv {
    root: Box<Expr>,
    rng: Box<Rng>,
}

impl Rv {
    pub fn next(&mut self) -> u32 {
        self.root.next(&mut self.rng)
    }

    pub fn prev(&self) -> u32 {
        self.root.prev()
    }

    pub fn done(&self) -> bool {
        self.root.done()
    }
}

impl fmt::Display for Rv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.root.fmt(f)
    }
}

pub struct Seed {
    pub value: [u8; 16],
}

impl Seed {
    /// Generates a 128-bit seed from a 32-bit seed
    ///
    /// This is done via two steps:
    ///
    /// 1. Create a low quality 128-bit seed (LQS)
    ///
    ///    This is done with simple bit manipulation of the 32-bit seed.
    ///
    /// 2. Create a higher quality 128-bit seed (HQS)
    ///
    ///    This is done by seeding an Rng with the LQS then using the Rng to generate the HQS.
    pub fn from_u32(seed: u32) -> Seed {
        let mut rng = XorShiftRng::from_seed(Seed::from_u32_array([
            seed ^ 0xa5a5_a5a5,
            seed ^ 0x5a5a_5a5a,
            seed ^ 0x5555_5555,
            seed ^ 0xaaaa_aaaa,
        ]).value);

        Seed::from_u32_array([rng.gen(), rng.gen(), rng.gen(), rng.gen()])
    }

    pub fn from_u32_array(x: [u32; 4]) -> Seed {
        Seed {
            value: [
                (x[0] >>  0) as u8,
                (x[0] >>  8) as u8,
                (x[0] >> 16) as u8,
                (x[0] >> 24) as u8,
                (x[1] >>  0) as u8,
                (x[1] >>  8) as u8,
                (x[1] >> 16) as u8,
                (x[1] >> 24) as u8,
                (x[2] >>  0) as u8,
                (x[2] >>  8) as u8,
                (x[2] >> 16) as u8,
                (x[2] >> 24) as u8,
                (x[3] >>  0) as u8,
                (x[3] >>  8) as u8,
                (x[3] >> 16) as u8,
                (x[3] >> 24) as u8,
            ],
        }
    }
}

pub struct Variables {
    refs: Vec<Box<Rv>>,
    indexes: LinkedHashMap<String, usize>,
}

pub struct VariablesIter<'a> {
    iter: ::linked_hash_map::Iter<'a, String, usize>,
    refs: &'a Vec<Box<Rv>>,
}

impl<'a> Iterator for VariablesIter<'a> {
    type Item = (&'a str, &'a Rv);

    fn next(&mut self) -> Option<(&'a str, &'a Rv)> {
        let next = self.iter.next()?;

        Some((next.0, &*self.refs[*next.1]))
    }
}

impl Variables {
    pub fn new() -> Variables {
        Variables {
            refs: Vec::new(),
            indexes: LinkedHashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, variable: Box<Rv>) {
        self.refs.push(variable);
        self.indexes.insert(name, self.refs.len() - 1);
    }

    pub fn last_mut(&mut self) -> Option<&mut Rv> {
        let variable = self.refs.last_mut()?;

        Some(&mut *variable)
    }

    pub fn get_index(&self, k: &str) -> Option<&usize> {
        self.indexes.get(k)
    }

    pub fn get_by_index(&mut self, index: usize) -> Option<&mut Rv> {
        let variable = self.refs.get_mut(index)?;

        Some(&mut *variable)
    }

    pub fn get(&mut self, k: &str) -> Option<&mut Rv> {
        let index = self.indexes.get(k)?;
        let variable = self.refs.get_mut(*index)?;
        Some(&mut *variable)
    }

    pub fn iter(&self) -> VariablesIter {
        VariablesIter {
            iter: self.indexes.iter(),
            refs: &self.refs,
        }
    }
}

impl fmt::Display for Variables {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (name, variable) in self.iter() {
            write!(f, "{} = ", name)?;
            variable.fmt(f)?;
            writeln!(f, ";")?;
        }

        Ok(())
    }
}

pub struct Context {
    pub variables: Variables,
    pub enums: LinkedHashMap<String, Enum>,
    pub seed: Seed,
    pub requires: RequirePaths,
}

impl Context {
    pub fn new() -> Context {
        Context {
            variables: Variables::new(),
            enums: LinkedHashMap::new(),
            seed: Seed::from_u32(0),
            requires: RequirePaths::new(),
        }
    }

    pub fn get(&mut self, name: &str) -> Option<&mut Rv> {
        self.variables.get(name)
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.variables.fmt(f)
    }
}

pub struct Enum {
    name: String,
    items: LinkedHashMap<String, u32>,
}

impl Enum {
    pub fn new(name: String, items: LinkedHashMap<String, u32>) -> Enum {
        Enum {
            name,
            items,
        }
    }
}

impl Context {
    pub fn transform_items(&mut self, items: &Vec<Box<ast::Node>>) {
        for item in items.iter() {
            match **item {
                ast::Node::Assignment(ref lhs, ref rhs) => {
                    self.transform_assignment(lhs, rhs);
                },
                ast::Node::Enum(ref name, ref items) => {
                    self.transform_enum(name, items);
                }
                _ => {},
            }
        }
    }

    pub fn transform_assignment(&mut self, lhs: &ast::Node, rhs: &ast::Node) {
        let mut identifier: String = "".into();

        if let ast::Node::Identifier(ref x) = *lhs {
            identifier = x.clone();
        }

        let mut rng = new_rng(&self.seed);
        let rv = Box::new(Rv {
            root: self.transform_expr(&mut rng, &rhs),
            rng: rng,
        });
        self.variables.insert(identifier, rv);
    }

    pub fn transform_enum(&mut self, name: &String, items: &Vec<Box<ast::Node>>) {
        let mut enum_items_map = LinkedHashMap::new();

        let mut next_implicit_value = 0;

        // FIXME Convert to .map()
        for item in items.iter() {
            if let ast::Node::EnumItem(ref name, ref value) = **item {
                if let Some(ref value) = *value {
                    if let ast::Node::Number(value) = **value {
                        // FIXME Check for existence
                        enum_items_map.insert(name.to_owned(), value);
                        next_implicit_value = value + 1;
                    } else {
                        panic!("Expected Number but found...FIXME");
                    }
                } else {
                    enum_items_map.insert(name.to_owned(), next_implicit_value);
                    next_implicit_value += 1;
                }
            } else {
                panic!("Expected EnumItem but found...FIXME");
            }
        }
        self.enums.insert(
            name.to_owned(),
            Enum::new(name.to_owned(), enum_items_map)
            );
    }

    pub fn transform_expr(&self, rng: &mut Rng, node: &ast::Node) -> Box<Expr> {
        match *node {
            ast::Node::Function(ref function, ref args) => {
                self.transform_function(rng, function, args)
            }
            ast::Node::Number(x) => Box::new(Value::new(x)),
            ast::Node::UnaryOperation(ref op, ref a) => {
                Box::new(
                    Unary::new(
                        op.clone(),
                        self.transform_expr(rng, a)
                        )
                    )
            },
            ast::Node::BinaryOperation(ref bx, ref op, ref by) => {
                Box::new(
                    Binary::new(
                        self.transform_expr(rng, bx),
                        op.clone(),
                        self.transform_expr(rng, by)
                    )
                )
            },
            ast::Node::EnumItemInst(ref a, ref b) => {
                if let Some(entry) = self.enums.get(a) {
                    if let Some(entry) = entry.items.get(b) {
                        Box::new(
                            Value::new(*entry)
                        )
                    } else {
                        panic!("Could not find enum value '{}' in enum '{}'", b, a);
                    }
                } else {
                    panic!("Could not find enum '{}'", a);
                }
            },
            _ => panic!("Not supported"),
        }
    }

    pub fn transform_function(&self, rng: &mut Rng, function: &ast::Function, args: &Vec<Box<ast::Node>>) -> Box<Expr> {
        match *function {
            ast::Function::Pattern => {
                Box::new(
                    Pattern::new(
                        args.into_iter()
                        .map(|arg| self.transform_expr(rng, &arg))
                        .collect()
                        )
                    )
            }
            ast::Function::Range => {
                let l = self.transform_expr(rng, &args[0]).next(rng);
                let r = self.transform_expr(rng, &args[1]).next(rng);

                Box::new(
                    RangeSequence::new(l, r)
                    )
            }
            ast::Function::Sample => {
                let mut children: Vec<Box<Expr>> = Vec::new();
                for arg in args.iter() {
                    if let ast::Node::EnumInst(ref name) = **arg {
                        if let Some(entry) = self.enums.get(name) {
                            for value in entry.items.values() {
                                children.push(
                                    Box::new(Value::new(*value))
                                );
                            }
                        } else {
                            panic!("Could not find enum '{}'", name);
                        }
                    } else {
                        children.push(self.transform_expr(rng, &arg));
                    }
                }

                Box::new(Sample::new(children))
            }
            ast::Function::WeightedSample => {
                Box::new(
                    WeightedSample::new(
                        args.into_iter()
                        .map(|arg|
                             if let ast::Node::WeightedPair(ref weight, ref node) = **arg {
                                 (*weight, self.transform_expr(rng, node))
                             } else {
                                 panic!("Expected WeightedPair but found ... FIXME");
                             }
                            )
                        .collect()
                        )
                    )
            }
        }
    }
}

fn new_rng(seed: &Seed) -> Box<Rng> {
    Box::new(XorShiftRng::from_seed(seed.value))
}


#[cfg(test)]
mod tests {
    use super::*;

    mod transform_expr {
        use super::*;

        use std::collections::HashMap;

        #[test]
        fn number() {
            let context = Context::new();
            let mut rng = new_rng(&Seed::from_u32(0));
            let ast = ast::Node::Number(4);
            let mut variable = context.transform_expr(&mut rng, &ast);

            assert_eq!(variable.next(&mut rng), 4);
        }

        #[test]
        fn range() {
            let context = Context::new();
            let mut rng = new_rng(&Seed::from_u32(0));
            let ast = ast::Node::Function(
                ast::Function::Range,
                vec![
                    Box::new(ast::Node::Number(3)),
                    Box::new(ast::Node::Number(4))
                ]
            );
            let mut variable = context.transform_expr(&mut rng, &ast);

            let mut values = HashMap::new();

            for _ in 0..10 {
                let value = variable.next(&mut rng);
                let entry = values.entry(value).or_insert(0);
                *entry += 1;
                assert!(value == 3 || value == 4);
            }

            assert!(values[&3] > 0);
            assert!(values[&4] > 0);
        }
    }
}
