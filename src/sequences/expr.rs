use ast::Opcode;

use super::Sequence;
use super::Value;

pub struct Expr {
    last: u32,
    operation: Opcode,
    l: Box<Sequence>,
    r: Box<Sequence>,
}

impl<'a> Expr {
    pub fn new(l: Box<Sequence>, operation: Opcode, r: Box<Sequence>) -> Expr {
        Expr {
            last: 0,
            operation: operation,
            l: l,
            r: r,
        }
    }
}

impl<'a> Sequence for Expr {
    fn next(&mut self) -> u32 {
        self.last = match self.operation {
            Opcode::Add => self.l.next() + self.r.next(),
            Opcode::Subtract => self.l.next() - self.r.next(),
            Opcode::Multiply => self.l.next() * self.r.next(),
            Opcode::Divide => self.l.next() / self.r.next(),
        };

        self.last
    }

    fn last(&self) -> u32 {
        self.last
    }
}

mod tests {
    use super::*;

    #[test]
    fn expr() {
        let v0 = Box::new(Value::new(1));
        let v1 = Box::new(Value::new(2));

        let mut expr = Expr::new(
            v0,
            Opcode::Add,
            v1,
        );

        assert_eq!(expr.next(), 3);
        assert_eq!(expr.next(), 3);
    }
}
