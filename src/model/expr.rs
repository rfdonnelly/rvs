use rand::Rng;
use std::fmt;

#[derive(Clone)]
pub struct ExprData {
    pub prev: u32,
    pub done: bool,
}

pub trait Expr: fmt::Display + ExprClone {
    fn next(&mut self, rng: &mut Rng) -> u32;

    fn prev(&self) -> u32 {
        self.data().prev
    }

    fn done(&self) -> bool {
        self.data().done
    }

    fn data(&self) -> &ExprData;
}

/// Used to implement clone for all implementors of Expr trait.
///
/// https://stackoverflow.com/a/30353928
pub trait ExprClone {
    fn clone_box(&self) -> Box<Expr>;
}

impl<T> ExprClone for T where T: 'static + Expr + Clone {
    fn clone_box(&self) -> Box<Expr> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Expr> {
    fn clone(&self) -> Box<Expr> {
        self.clone_box()
    }
}
