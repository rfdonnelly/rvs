use ast::Opcode;

use super::Sequence;

pub struct Expr {
    prev: u32,
    done: bool,
    operation: Opcode,
    l: Box<Sequence>,
    r: Box<Sequence>,
}

impl<'a> Expr {
    pub fn new(l: Box<Sequence>, operation: Opcode, r: Box<Sequence>) -> Expr {
        Expr {
            prev: 0,
            done: false,
            operation: operation,
            l: l,
            r: r,
        }
    }
}

impl<'a> Sequence for Expr {
    fn next(&mut self) -> u32 {
        let (l, r) = (self.l.next(), self.r.next());

        self.done = self.l.done() || self.r.done();

        self.prev = match self.operation {
            Opcode::Or => l | r,
            Opcode::Xor => l ^ r,
            Opcode::And => l & r,
            Opcode::Shl => l << r,
            Opcode::Shr => l >> r,
            Opcode::Add => l + r,
            Opcode::Sub => l - r,
            Opcode::Mul => l * r,
            Opcode::Div => l / r,
            Opcode::Mod => l % r,
        };

        self.prev
    }

    fn prev(&self) -> u32 {
        self.prev
    }

    fn done(&self) -> bool {
        self.done
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sequences::Value;

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
