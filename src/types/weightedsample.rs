use std::fmt;
use rand::Rng;
use rand::sequences::Weighted;
use rand::sequences::WeightedChoice;
use rand::Distribution;

use types::Expr;
use types::ExprData;
use types::Context;

pub struct WeightedSample {
    data: ExprData,
    children: Vec<(u32, Box<Expr>)>,
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
            weighted_choice,
        }
    }
}

impl Expr for WeightedSample {
    fn next(&mut self, rng: &mut Rng, context: &Context) -> u32 {
        let index = self.weighted_choice.sample(rng);
        self.data.prev = self.children[index].1.next(rng, context);

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
