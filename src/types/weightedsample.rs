use transform::CrateRng;
use model::{Expr, ExprData};

use rand::sequences::{Weighted, WeightedChoice};
use rand::Distribution;
use std::fmt;

#[derive(Clone)]
pub struct WeightedSample {
    data: ExprData,
    children: Vec<(u32, Box<Expr>)>,
    current_child: Option<usize>,
    weighted_choice: WeightedChoice<usize>,
}

impl WeightedSample {
    pub fn new(children: Vec<(u32, Box<Expr>)>) -> WeightedSample {
        let mut weights = Vec::new();
        for (i, child) in children.iter().enumerate() {
            weights.push(Weighted { weight: child.0, item: i });
        }

        let weighted_choice = WeightedChoice::new(weights);

        WeightedSample {
            data: ExprData {
                prev: 0,
                done: false,
            },
            children,
            current_child: None,
            weighted_choice,
        }
    }
}

impl Expr for WeightedSample {
    fn next(&mut self, rng: &mut CrateRng) -> u32 {
        let index = match self.current_child {
            Some(index) => index,
            None => self.weighted_choice.sample(rng),
        };

        self.data.prev = self.children[index].1.next(rng);
        self.data.done = self.children[index].1.done();
        self.current_child = match self.data.done {
            true => None,
            false => Some(index),
        };

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for WeightedSample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for child in self.children.iter() {
            write!(f, "{}: {}, ", child.0, child.1)?;
        }
        write!(f, "}}")
    }
}
