use std::fmt;
use std::fmt::Write;
use rand::Rng;

use rvs_parser::ast;
use model::{Expr, ExprData};

use std::num::Wrapping;

#[derive(Clone)]
pub struct Binary {
    data: ExprData,
    operation: ast::BinaryOpcode,
    operands: (Box<Expr>, Box<Expr>),
    done: (bool, bool),
}

#[derive(Clone)]
pub struct Unary {
    data: ExprData,
    operation: ast::UnaryOpcode,
    operand: Box<Expr>,
}

impl Binary {
    pub fn new(l: Box<Expr>, operation: ast::BinaryOpcode, r: Box<Expr>) -> Binary {
        Binary {
            data: ExprData {
                prev: 0,
                done: false,
            },
            operation: operation,
            operands: (l, r),
            done: (false, false),
        }
    }
}

impl Expr for Binary {
    fn next(&mut self, rng: &mut Rng) -> u32 {
        let l = self.operands.0.next(rng);
        let r = self.operands.1.next(rng);

        self.done.0 |= self.operands.0.done();
        self.done.1 |= self.operands.1.done();
        self.data.done = self.done.0 && self.done.1;

        self.data.prev = match self.operation {
            ast::BinaryOpcode::Or => l | r,
            ast::BinaryOpcode::Xor => l ^ r,
            ast::BinaryOpcode::And => l & r,
            ast::BinaryOpcode::Shl => (Wrapping(l) << (r as usize)).0,
            ast::BinaryOpcode::Shr => (Wrapping(l) >> (r as usize)).0,
            ast::BinaryOpcode::Add => (Wrapping(l) + Wrapping(r)).0,
            ast::BinaryOpcode::Sub => (Wrapping(l) - Wrapping(r)).0,
            ast::BinaryOpcode::Mul => (Wrapping(l) * Wrapping(r)).0,
            ast::BinaryOpcode::Div => l / r,
            ast::BinaryOpcode::Mod => l % r,
        };

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char('(')?;
        self.operands.0.fmt(f)?;
        f.write_char(' ')?;
        self.operation.fmt(f)?;
        f.write_char(' ')?;
        self.operands.1.fmt(f)?;
        f.write_char(')')
    }
}

impl Unary {
    pub fn new(operation: ast::UnaryOpcode, operand: Box<Expr>) -> Unary {
        Unary {
            data: ExprData {
                prev: 0,
                done: false,
            },
            operation,
            operand,
        }
    }
}

impl Expr for Unary {
    fn next(&mut self, rng: &mut Rng) -> u32 {
        let operand = self.operand.next(rng);

        self.data.done = self.operand.done();

        self.data.prev = match self.operation {
            ast::UnaryOpcode::Inv => !operand,
            ast::UnaryOpcode::Neg => (Wrapping(!operand) + Wrapping(1)).0,
        };

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.operation.fmt(f)?;
        self.operand.fmt(f)
    }
}
