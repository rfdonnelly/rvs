use super::rand::{CrateRng, Seed};
use super::enumeration::Enum;
use super::symbols::{Symbol, Symbols};

use crate::model::{Expr, Model, Variable, VariableRef};
use crate::types::{Binary, Done, Next, Once, Pattern, Prev, Range, Sequence, Unary, Value,
            WeightedWithReplacement, WeightedWithoutReplacement};
use crate::error::{TransformError, TransformResult};

use rvs_parser::ast;

use indexmap::IndexMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Transform {
    seed: Seed,
    symbols: Symbols,
}

impl Transform {
    pub fn new(seed: Seed) -> Transform {
        Transform {
            seed,
            symbols: Symbols::new(),
        }
    }

    pub fn transform(
        &mut self,
        model: &mut Model,
        nodes: &[Box<ast::Node>],
    ) -> TransformResult<()> {
        for node in nodes {
            match **node {
                ast::Node::Variable(ref name, ref expr) => {
                    let variable = self.transform_variable(model, expr)?;
                    let variable_index = model.add_variable(name, variable);
                    self.symbols.insert_variable(name, variable_index);
                }
                ast::Node::Enum(ref name, ref items) => {
                    self.transform_enum(name, items)?;
                }
                _ => {
                    return Err(TransformError::new(format!(
                        "expected Variable or Enum but found {:?}",
                        node
                    )));
                }
            }
        }

        Ok(())
    }

    fn transform_variable(&self, model: &Model, expr: &ast::Node) -> TransformResult<VariableRef> {
        let mut rng = self.seed.to_rng();
        let expr = self.transform_expr(model, &mut rng, expr)?;
        let variable = Rc::new(RefCell::new(Box::new(Variable::new(expr, rng))));

        Ok(variable)
    }

    fn transform_enum(&mut self, name: &str, items: &[Box<ast::Node>]) -> TransformResult<()> {
        if self.symbols.contains(name) {
            return Err(TransformError::new(format!(
                "Symbol '{}' already exists",
                name
            )));
        }

        let mut enum_members_map = IndexMap::new();
        let mut next_implicit_value = 0;

        // FIXME change to drain()?
        for item in items {
            if let ast::Node::EnumMember(ref member_name, ref value) = **item {
                let full_name = format!("{}::{}", name, member_name);
                if let Some(ref value) = *value {
                    if let ast::Node::Number(value) = **value {
                        // FIXME Check for existence
                        enum_members_map.insert(member_name.to_owned(), value);
                        self.symbols.insert_enum_member(full_name, value);
                        next_implicit_value = value + 1;
                    } else {
                        return Err(TransformError::new(format!(
                            "Expected Number but found {:?}",
                            **value
                        )));
                    }
                } else {
                    enum_members_map.insert(member_name.to_owned(), next_implicit_value);
                    self.symbols
                        .insert_enum_member(full_name, next_implicit_value);
                    next_implicit_value += 1;
                }
            } else {
                return Err(TransformError::new(format!(
                    "Expected EnumMember but found {:?}",
                    **item
                )));
            }
        }
        self.symbols.insert_enum(name, Enum::new(enum_members_map));

        Ok(())
    }

    fn transform_expr(
        &self,
        model: &Model,
        rng: &mut CrateRng,
        node: &ast::Node,
    ) -> TransformResult<Box<dyn Expr>> {
        match *node {
            ast::Node::Type(ref typ, ref args) => self.transform_type(model, rng, typ, args),
            ast::Node::Weighted(ref replacement, ref args) =>
                self.transform_weighted(model, rng, replacement, args),
            ast::Node::Number(x) => Ok(Box::new(Value::new(x))),
            ast::Node::UnaryOperation(ref op, ref a) => Ok(Box::new(Unary::new(
                op.clone(),
                self.transform_expr(model, rng, a)?,
            ))),
            ast::Node::BinaryOperation(ref bx, ref op, ref by) => Ok(Box::new(Binary::new(
                self.transform_expr(model, rng, bx)?,
                op.clone(),
                self.transform_expr(model, rng, by)?,
            ))),
            ast::Node::RIdentifier(ref name, ref method) => {
                match self.symbols.get(name) {
                    Some(symbol) => {
                        match *symbol {
                            Symbol::EnumMember(ref value) => {
                                Ok(Box::new(Value::new(*value)))
                            }
                            Symbol::Variable(ref index) => {
                                self.transform_r_variable(model, name, *index, method)
                            }
                            Symbol::Enum(_) => {
                                Err(TransformError::new(format!(
                                    "Expected a Variable or EnumMember identifier but found Enum identifer '{}'",
                                    name
                                )))
                            }
                        }
                    }
                    None => {
                        Err(TransformError::new(format!(
                            "Could not find symbol '{}'",
                            name
                        )))
                    }
                }
            }
            _ => Err(TransformError::new(format!(
                "Expected (Type|Number|UnaryOperation|BinaryOperation|Identifier) but found {:?}",
                *node
            ))),
        }
    }

    fn transform_r_variable(
        &self,
        model: &Model,
        variable_name: &str,
        variable_index: usize,
        method: &ast::VariableMethod,
    ) -> TransformResult<Box<dyn Expr>> {
        match model.get_variable_by_index(variable_index) {
            Some(variable) => match *method {
                ast::VariableMethod::Next => {
                    Ok(Box::new(Next::new(variable_name, Rc::downgrade(variable))))
                }
                ast::VariableMethod::Prev => {
                    Ok(Box::new(Prev::new(variable_name, Rc::downgrade(variable))))
                }
                ast::VariableMethod::Copy => Ok(variable.borrow().clone_expr()),
            },
            None => Err(TransformError::new(format!(
                "Could not find variable '{}'",
                variable_name
            ))),
        }
    }

    fn transform_args(
        &self,
        model: &Model,
        rng: &mut CrateRng,
        args: &[Box<ast::Node>],
    ) -> TransformResult<Vec<Box<dyn Expr>>> {
        let mut arg_exprs: Vec<Box<dyn Expr>> = Vec::new();
        for arg in args {
            arg_exprs.push(self.transform_expr(model, rng, arg)?);
        }

        Ok(arg_exprs)
    }

    fn transform_weighted(
        &self,
        model: &Model,
        rng: &mut CrateRng,
        replacement: &ast::Replacement,
        args: &[Box<ast::Node>],
    ) -> TransformResult<Box<dyn Expr>> {
        let mut weights: Vec<u32> = Vec::new();
        let mut children: Vec<Box<dyn Expr>> = Vec::new();
        for arg in args {
            match **arg {
                ast::Node::Type(ast::Type::Expand, ref args) => {
                    let mut expr = self.transform_expr(model, rng, &args[0])?;

                    if args.len() == 1 {
                        while !expr.done() {
                            weights.push(1);
                            children.push(Box::new(Value::new(expr.next(rng))));
                        }
                    } else {
                        let mut count = self.transform_expr(model, rng, &args[1])?;
                        for _ in 0..count.next(rng) {
                            weights.push(1);
                            children.push(Box::new(Value::new(expr.next(rng))));
                        }
                    }
                }
                ast::Node::WeightedSample(ref weight, ref node) => {
                    weights.push(*weight);
                    children.push(self.transform_expr(model, rng, node)?);
                }
                _ => {
                    return Err(TransformError::new(format!(
                        "Expected WeightedSample but found {:?}",
                        **arg
                    )));
                }
            }
        }

        match *replacement {
            ast::Replacement::With => Ok(Box::new(WeightedWithReplacement::new(weights, children))),
            ast::Replacement::Without => Ok(Box::new(WeightedWithoutReplacement::new(
                weights,
                children,
                rng,
            ))),
        }
    }

    fn transform_type(
        &self,
        model: &Model,
        rng: &mut CrateRng,
        typ: &ast::Type,
        args: &[Box<ast::Node>],
    ) -> TransformResult<Box<dyn Expr>> {
        match *typ {
            ast::Type::Pattern => Ok(Box::new(Pattern::new(self.transform_args(
                model,
                rng,
                args,
            )?))),
            ast::Type::Sequence => {
                let args = self.transform_args(model, rng, args)?;

                Ok(Box::new(Sequence::new(args, rng)))
            }
            ast::Type::Range => {
                let l = self.transform_expr(model, rng, &args[0])?.next(rng);
                let r = self.transform_expr(model, rng, &args[1])?.next(rng);

                // Elide the range for case when limits are equal
                //
                // The underlying rand::distributions::Range treats this case as an error.  We
                // don't want an error so catch and handle gracefully.
                if l == r {
                    Ok(Box::new(Value::new(1)))
                } else {
                    Ok(Box::new(Range::new(l, r)))
                }
            }
            ast::Type::Expand => Err(TransformError::new("Expand() must be inside {}".to_owned())),
            ast::Type::Done => {
                let expr = self.transform_expr(model, rng, &*args[0])?;
                Ok(Box::new(Done::new(expr)))
            }
            ast::Type::Once => {
                let expr = self.transform_expr(model, rng, &*args[0])?;
                Ok(Box::new(Once::new(expr)))
            }
        }
    }
}
