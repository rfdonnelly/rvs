use std::fmt;
use std::fmt::Write;
use rand::Rng;

use ast::BinaryOpcode;
use ast::UnaryOpcode;
use types::Expr;
use types::ExprData;

pub struct Binary {
    data: ExprData,
    operation: BinaryOpcode,
    l: Box<Expr>,
    r: Box<Expr>,
}

pub struct Unary {
    data: ExprData,
    operation: UnaryOpcode,
    operand: Box<Expr>,
}

impl Binary {
    pub fn new(l: Box<Expr>, operation: BinaryOpcode, r: Box<Expr>) -> Binary {
        Binary {
            data: ExprData {
                prev: 0,
                done: false,
            },
            operation: operation,
            l: l,
            r: r,
        }
    }
}

impl Expr for Binary {
    fn next(&mut self, rng: &mut Rng) -> u32 {
        let (l, r) = (self.l.next(rng), self.r.next(rng));

        self.data.done = self.l.done() || self.r.done();

        self.data.prev = match self.operation {
            BinaryOpcode::Or => l | r,
            BinaryOpcode::Xor => l ^ r,
            BinaryOpcode::And => l & r,
            BinaryOpcode::Shl => l << r,
            BinaryOpcode::Shr => l >> r,
            BinaryOpcode::Add => l + r,
            BinaryOpcode::Sub => l - r,
            BinaryOpcode::Mul => l * r,
            BinaryOpcode::Div => l / r,
            BinaryOpcode::Mod => l % r,
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
        self.l.fmt(f)?;
        f.write_char(' ')?;
        self.operation.fmt(f)?;
        f.write_char(' ')?;
        self.r.fmt(f)?;
        f.write_char(')')
    }
}

impl Unary {
    pub fn new(operation: UnaryOpcode, operand: Box<Expr>) -> Unary {
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
            UnaryOpcode::Neg => !operand,
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

#[cfg(test)]
mod tests {
    use super::*;
    use types::Value;
    use types::new_rng;
    use types::Seed;

    #[test]
    fn binary() {
        let mut rng = new_rng(&Seed::from_u32(0));
        let v0 = Box::new(Value::new(1));
        let v1 = Box::new(Value::new(2));

        let mut binary = Binary::new(
            v0,
            BinaryOpcode::Add,
            v1,
        );

        assert_eq!(binary.next(&mut rng), 3);
        assert_eq!(binary.next(&mut rng), 3);
    }
}
