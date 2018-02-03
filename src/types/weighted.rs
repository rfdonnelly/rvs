use transform::CrateRng;
use model::{Expr, ExprData};

use rand::Rng;
use rand::distributions::{Range, Sample};
use std::fmt;

#[derive(Clone)]
pub struct Weighted {
    data: ExprData,
    children: Vec<(u32, Box<Expr>)>,
    current_child: Option<usize>,
    weighted_indexes: WeightedIndexes,
}

impl Weighted {
    pub fn new(children: Vec<(u32, Box<Expr>)>) -> Weighted {
        let weights: Vec<u32> = children.iter().map(|child| child.0).collect();

        Weighted {
            data: ExprData {
                prev: 0,
                done: false,
            },
            children,
            current_child: None,
            weighted_indexes: WeightedIndexes::new(weights),
        }
    }
}

impl Expr for Weighted {
    fn next(&mut self, rng: &mut CrateRng) -> u32 {
        let index = match self.current_child {
            Some(index) => index,
            None => self.weighted_indexes.sample(rng),
        };

        self.data.prev = self.children[index].1.next(rng);
        self.data.done = self.children[index].1.done();
        self.current_child = if self.data.done {
            None
        } else {
            Some(index)
        };

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for Weighted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for child in &self.children {
            write!(f, "{}: {}, ", child.0, child.1)?;
        }
        write!(f, "}}")
    }
}

#[derive(Clone)]
struct WeightedIndexes {
    weights: Vec<u32>,
    range: Range<u32>,
}

impl WeightedIndexes {
    pub fn new(weights: Vec<u32>) -> WeightedIndexes {
        let weights = WeightedIndexes::accumulate(weights);
        let total_weight = *weights.last().unwrap();

        WeightedIndexes {
            weights,
            range: Range::new(0, total_weight),
        }
    }

    fn accumulate(mut weights: Vec<u32>) -> Vec<u32> {
        let mut previous = 0;

        for weight in &mut weights {
            // FIXME: Check for overflow
            *weight += previous;
            previous = *weight;
        }

        weights
    }

    fn select(&self, value: u32) -> usize {
        // FIXME: Ensure value is between 0 and self.weights.len() - 1
        for (i, weight) in self.weights.iter().enumerate() {
            if value < *weight {
                return i;
            }
        }

        panic!("unreachable");
    }

    pub fn sample<R: Rng>(&mut self, rng: &mut R) -> usize {
        let value = self.range.sample(rng);
        self.select(value)
    }
}
