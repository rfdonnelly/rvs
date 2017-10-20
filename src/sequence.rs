use ast::Opcode;

pub trait Sequence {
    fn next(&mut self) -> u32;
    fn last(&self) -> u32;
}

pub struct Value {
    last: u32,
}

pub struct Expr<'a> {
    last: u32,
    operation: Opcode,
    l: &'a mut Sequence,
    r: &'a mut Sequence,
}

impl Value {
    pub fn new(value: u32) -> Value {
        Value {
            last: value,
        }
    }
}

impl Sequence for Value {
    fn next(&mut self) -> u32 {
        self.last
    }

    fn last(&self) -> u32 {
        self.last
    }
}

impl<'a> Expr<'a> {
    pub fn new(l: &'a mut Sequence, operation: Opcode, r: &'a mut Sequence) -> Expr<'a> {
        Expr {
            last: 0,
            operation: operation,
            l: l,
            r: r,
        }
    }
}

impl<'a> Sequence for Expr<'a> {
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
        let mut v0 = Value::new(1);
        let mut v1 = Value::new(2);

        let mut expr = Expr::new(
            &mut v0,
            Opcode::Add,
            &mut v1,
        );

        assert_eq!(expr.next(), 3);
        assert_eq!(expr.next(), 3);
    }
}
