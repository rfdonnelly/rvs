use std::fmt;
use std::fmt::Write;
use rand::Rng;

use ast::Opcode;
use types::Rv;
use types::RvData;

pub struct Expr {
    data: RvData,
    operation: Opcode,
    l: Box<Rv>,
    r: Box<Rv>,
}

impl Expr {
    pub fn new(l: Box<Rv>, operation: Opcode, r: Box<Rv>) -> Expr {
        Expr {
            data: RvData {
                prev: 0,
                done: false,
            },
            operation: operation,
            l: l,
            r: r,
        }
    }
}

impl Rv for Expr {
    fn next(&mut self, rng: &mut Rng) -> u32 {
        let (l, r) = (self.l.next(rng), self.r.next(rng));

        self.data.done = self.l.done() || self.r.done();

        self.data.prev = match self.operation {
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

        self.data.prev
    }

    fn data(&self) -> &RvData {
        &self.data
    }
}

impl fmt::Display for Expr {
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

#[cfg(test)]
mod tests {
    use super::*;
    use types::Value;
    use types::new_rng;
    use types::Seed;

    #[test]
    fn expr() {
        let mut rng = new_rng(&Seed::from_u32(0));
        let v0 = Box::new(Value::new(1));
        let v1 = Box::new(Value::new(2));

        let mut expr = Expr::new(
            v0,
            Opcode::Add,
            v1,
        );

        assert_eq!(expr.next(&mut rng), 3);
        assert_eq!(expr.next(&mut rng), 3);
    }
}
