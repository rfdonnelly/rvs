use transform::CrateRng;
use model::{Expr, ExprData};

use rand::Rng;
use rand::distributions::{Range, Sample};
use std::fmt;

#[derive(Clone)]
pub struct WeightedWithReplacement {
    data: ExprData,
    weights: Vec<u32>,
    children: Vec<Box<Expr>>,
    pool: Vec<usize>,
    current_pool_entry: Option<usize>,
    range: Range<usize>,
}

impl WeightedWithReplacement {
    pub fn new(weights: Vec<u32>, children: Vec<Box<Expr>>) -> WeightedWithReplacement {
        let mut pool: Vec<usize> = Vec::new();
        for (i, weight) in weights.iter().enumerate() {
            for _ in 0..*weight {
                pool.push(i);
            }
        }

        WeightedWithReplacement {
            data: ExprData {
                prev: 0,
                done: false,
            },
            weights,
            children,
            current_pool_entry: None,
            range: Range::new(0, pool.len()),
            pool,
        }
    }
}

impl Expr for WeightedWithReplacement {
    fn next(&mut self, rng: &mut CrateRng) -> u32 {
        let index = match self.current_pool_entry {
            Some(index) => index,
            None => self.range.sample(rng),
        };

        self.data.prev = self.children[self.pool[index]].next(rng);
        self.data.done = self.children[self.pool[index]].done();
        self.current_pool_entry = if self.data.done { None } else { Some(index) };

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for WeightedWithReplacement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r{{")?;
        for (i, child) in self.children.iter().enumerate() {
            write!(f, "{}: {}, ", self.weights[i], child)?;
        }
        write!(f, "}}")
    }
}

// Implements weighted sampling without replacement.
//
// Each sample gets a number of entries added to the pool.  The number of entries is equal to the
// weight.  On creation, the pool is shuffled.  The pool is then iterated over.  When the pool is
// fully iterated over, the pool is shuffled again and iteration is reset.
#[derive(Clone)]
pub struct WeightedWithoutReplacement {
    data: ExprData,
    weights: Vec<u32>,
    children: Vec<Box<Expr>>,
    current_child: usize,
    visit_order: Vec<usize>,
}

impl WeightedWithoutReplacement {
    pub fn new(
        weights: Vec<u32>,
        children: Vec<Box<Expr>>,
        rng: &mut CrateRng,
    ) -> WeightedWithoutReplacement {
        let mut visit_order: Vec<usize> = Vec::new();
        for (i, weight) in weights.iter().enumerate() {
            for _ in 0..*weight {
                visit_order.push(i);
            }
        }
        rng.shuffle(&mut visit_order);

        WeightedWithoutReplacement {
            data: ExprData {
                prev: 0,
                done: false,
            },
            weights,
            children,
            current_child: 0,
            visit_order,
        }
    }
}

impl Expr for WeightedWithoutReplacement {
    fn next(&mut self, rng: &mut CrateRng) -> u32 {
        let index = self.visit_order[self.current_child];
        self.data.prev = self.children[index].next(rng);

        if self.children[index].done() {
            self.current_child += 1;
            if self.current_child == self.visit_order.len() {
                self.current_child = 0;
                self.data.done = true;
                rng.shuffle(&mut self.visit_order);
            }
        } else {
            self.data.done = false;
        }

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for WeightedWithoutReplacement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for (i, child) in self.children.iter().enumerate() {
            write!(f, "{}: {}, ", self.weights[i], child)?;
        }
        write!(f, "}}")
    }
}
