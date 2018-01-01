use super::seed::Seed;
use super::enumeration::Enum;

use model::{
    Expr,
    Variable,
    VariableRef,
    Model,
};
use types::{
    Value,
    Unary,
    Binary,
    Pattern,
    Sequence,
    Range,
    Sample,
    WeightedSample,
    Next,
    Prev,
};
use error::{
    TransformError,
    TransformResult,
};

use rvs_parser::ast;

use rand::Rng;
use linked_hash_map::LinkedHashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Transform {
    seed: Seed,
    enums: HashMap<String, Enum>,
}

impl Transform {
    pub fn new(seed: Seed) -> Transform {
        Transform {
            seed,
            enums: HashMap::new(),
        }
    }

    pub fn transform(
        &mut self,
        nodes: &Vec<Box<ast::Node>>
    ) -> TransformResult<Model> {
        let mut model = Model::new();

        for node in nodes {
            match **node {
                ast::Node::Variable(ref name, ref expr) => {
                    let variable = self.transform_variable(&model, expr)?;
                    model.add_variable(name, variable);
                }
                ast::Node::Enum(ref name, ref items) => {
                    self.transform_enum(name, items)?;
                }
                _ => {
                    return Err(TransformError::new(format!(
                        "expected Variable or Enum but found {:?}", node)));
                },
            }
        }

        Ok(model)
    }

    fn transform_variable(
        &self,
        model: &Model,
        expr: &ast::Node
    ) -> TransformResult<VariableRef> {
        let mut rng = self.seed.to_rng();
        let expr = self.transform_expr(model, &mut rng, expr)?;
        let variable = Rc::new(RefCell::new(Box::new(Variable::new(expr, rng))));

        Ok(variable)
    }

    fn transform_enum(
        &mut self,
        name: &str,
        items: &Vec<Box<ast::Node>>
    ) -> TransformResult<()> {
        let mut enum_items_map = LinkedHashMap::new();

        let mut next_implicit_value = 0;

        // FIXME change to drain()?
        for item in items {
            if let ast::Node::EnumItem(ref name, ref value) = **item {
                if let Some(ref value) = *value {
                    if let ast::Node::Number(value) = **value {
                        // FIXME Check for existence
                        enum_items_map.insert(name.to_owned(), value);
                        next_implicit_value = value + 1;
                    } else {
                        return Err(TransformError::new(format!(
                                    "Expected Number but found {:?}", **value
                                    )));
                    }
                } else {
                    enum_items_map.insert(name.to_owned(), next_implicit_value);
                    next_implicit_value += 1;
                }
            } else {
                return Err(TransformError::new(format!(
                            "Expected EnumItem but found {:?}", **item
                            )));
            }
        }
        self.enums.insert(name.to_owned(), Enum::new(enum_items_map));

        Ok(())
    }

    fn transform_expr(
        &self,
        model: &Model,
        rng: &mut Rng,
        node: &ast::Node
    ) -> TransformResult<Box<Expr>> {
        match *node {
            ast::Node::Function(ref function, ref args) => {
                self.transform_function(model, rng, function, args)
            },
            ast::Node::Number(x) => {
                Ok(Box::new(Value::new(x)))
            },
            ast::Node::UnaryOperation(ref op, ref a) => {
                Ok(Box::new(
                        Unary::new(
                            op.clone(),
                            self.transform_expr(model, rng, a)?
                            )
                        ))
            },
            ast::Node::BinaryOperation(ref bx, ref op, ref by) => {
                Ok(Box::new(
                        Binary::new(
                            self.transform_expr(model, rng, bx)?,
                            op.clone(),
                            self.transform_expr(model, rng, by)?
                            )
                        ))
            },
            ast::Node::EnumItemInst(ref a, ref b) => {
                if let Some(entry) = self.enums.get(a) {
                    if let Some(entry) = entry.items.get(b) {
                        Ok(Box::new(Value::new(*entry)))
                    } else {
                        Err(TransformError::new(format!(
                                    "Could not find enum value '{}' in enum '{}'", b, a
                                    )))
                    }
                } else {
                    Err(TransformError::new(format!(
                                "Could not find enum '{}'", a
                                )))
                }
            },
            ast::Node::VariableInst(ref name, ref method) => {
                self.transform_variable_inst(model, name, method)
            },
            _ => {
                Err(TransformError::new(format!(
                    "Expected (Function|Number|UnaryOperation|BinaryOperation|EnumItemInst) but found {:?}",
                    *node)))
            }
        }
    }

    fn transform_variable_inst(
        &self,
        model: &Model,
        name: &str,
        method: &ast::VariableMethod
    ) -> TransformResult<Box<Expr>> {
        match model.get_variable_by_name(name) {
            Some(variable) => {
                match *method {
                    ast::VariableMethod::Next => {
                        Ok(Box::new(Next::new(name, Rc::downgrade(variable))))
                    },
                    ast::VariableMethod::Prev => {
                        Ok(Box::new(Prev::new(name, Rc::downgrade(variable))))
                    },
                    ast::VariableMethod::Copy => {
                        Ok(variable.borrow().clone_expr())
                    },
                }
            },
            None => {
                Err(TransformError::new(format!(
                            "Could not find variable '{}'", name
                            )))
            },
        }
    }

    fn transform_args(
        &self,
        model: &Model,
        rng: &mut Rng,
        args: &Vec<Box<ast::Node>>
    ) -> TransformResult<Vec<Box<Expr>>> {
        let mut arg_exprs: Vec<Box<Expr>> = Vec::new();
        for arg in args {
            arg_exprs.push(self.transform_expr(model, rng, &arg)?);
        }

        Ok(arg_exprs)
    }

    fn transform_function(
        &self,
        model: &Model,
        rng: &mut Rng,
        function: &ast::Function,
        args: &Vec<Box<ast::Node>>
    ) -> TransformResult<Box<Expr>> {
        match *function {
            ast::Function::Pattern => {
                Ok(Box::new(Pattern::new(self.transform_args(model, rng, args)?)))
            }
            ast::Function::Sequence => {
                let args = self.transform_args(model, rng, args)?.iter_mut().map(|arg| {
                    arg.next(rng)
                }).collect();

                Ok(Box::new(Sequence::new(args)))
            }
            ast::Function::Range => {
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
                            return Err(TransformError::new(format!(
                                        "Could not find enum '{}'", name
                                        )));
                        }
                    } else {
                        children.push(self.transform_expr(model, rng, &arg)?);
                    }
                }

                Ok(Box::new(Sample::new(children)))
            }
            ast::Function::WeightedSample => {
                let mut pairs: Vec<(u32, Box<Expr>)> = Vec::new();
                for arg in args {
                    if let ast::Node::WeightedPair(ref weight, ref node) = **arg {
                        pairs.push((*weight, self.transform_expr(model, rng, node)?));
                    } else {
                        return Err(TransformError::new(format!(
                                    "Expected WeightedPair but found {:?}", **arg)));
                    }
                }

                Ok(Box::new(WeightedSample::new(pairs)))
            }
        }
    }
}
