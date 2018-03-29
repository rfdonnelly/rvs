use transform::CrateRng;
use model::{Expr, ExprData};

use rand::Rng;
use rand::distributions::Distribution;
use rand::distributions::range::{Range, RangeInt};
use std::fmt;

#[derive(Clone)]
pub struct WeightedWithReplacement {
    data: ExprData,
    weights: Vec<u32>,
    children: Vec<Box<Expr>>,
    range: Range<RangeInt<usize>>,
    pool: Vec<usize>,
    pool_index: Option<usize>,
}

impl WeightedWithReplacement {
    pub fn new(weights: Vec<u32>, children: Vec<Box<Expr>>) -> WeightedWithReplacement {
        let pool = populate_pool(&weights);

        WeightedWithReplacement {
            data: Default::default(),
            weights,
            children,
            range: Range::new(0, pool.len()),
            pool,
            pool_index: None,
        }
    }
}

impl Expr for WeightedWithReplacement {
    fn next(&mut self, rng: &mut CrateRng) -> u32 {
        let pool_index = match self.pool_index {
            Some(pool_index) => pool_index,
            None => self.range.sample(rng),
        };
        let child_index = self.pool[pool_index];

        self.data.prev = self.children[child_index].next(rng);
        self.data.done = self.children[child_index].done();
        self.pool_index = if self.data.done {
            None
        } else {
            Some(pool_index)
        };

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

#[derive(Clone)]
pub struct WeightedWithoutReplacement {
    data: ExprData,
    weights: Vec<u32>,
    children: Vec<Box<Expr>>,
    pool: Vec<usize>,
    pool_index: usize,
}

impl WeightedWithoutReplacement {
    pub fn new(
        weights: Vec<u32>,
        children: Vec<Box<Expr>>,
        rng: &mut CrateRng,
    ) -> WeightedWithoutReplacement {
        let mut pool = populate_pool(&weights);
        rng.shuffle(&mut pool);

        WeightedWithoutReplacement {
            data: Default::default(),
            weights,
            children,
            pool,
            pool_index: 0,
        }
    }
}

impl Expr for WeightedWithoutReplacement {
    fn next(&mut self, rng: &mut CrateRng) -> u32 {
        let child_index = self.pool[self.pool_index];
        self.data.prev = self.children[child_index].next(rng);

        self.data.done = false;
        if self.children[child_index].done() {
            self.pool_index += 1;
            if self.pool_index == self.pool.len() {
                self.pool_index = 0;
                self.data.done = true;
                rng.shuffle(&mut self.pool);
            }
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

/// Converts weights into a pool of indexes
///
/// The index of each weight is added to the pool <weight> times.
fn populate_pool(weights: &[u32]) -> Vec<usize> {
    let mut pool: Vec<usize> = Vec::new();
    for (i, weight) in weights.iter().enumerate() {
        for _ in 0..*weight {
            pool.push(i);
        }
    }

    pool
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_populate_pool() {
        assert_eq!(populate_pool(&[3, 1, 2]), [0, 0, 0, 1, 2, 2]);
    }
}
