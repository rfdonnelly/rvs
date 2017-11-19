use std::fmt;
use rand::Rng;
use rand::sequences::Weighted;
use rand::sequences::WeightedChoice;
use rand::Distribution;

use types::Rv;
use types::RvData;

pub struct WeightedSample {
    data: RvData,
    children: Vec<(u32, Box<Rv>)>,
    weighted_choice: WeightedChoice<usize>,
}

impl WeightedSample {
    pub fn new(children: Vec<(u32, Box<Rv>)>) -> WeightedSample {
        let mut weights = Vec::new();
        for (i, child) in children.iter().enumerate() {
            weights.push(Weighted { weight: child.0, item: i });
        }

        let weighted_choice = WeightedChoice::new(weights);

        WeightedSample {
            data: RvData {
                prev: 0,
                done: false,
            },
            children,
            weighted_choice,
        }
    }
}

impl Rv for WeightedSample {
    fn next(&mut self, rng: &mut Rng) -> u32 {
        let index = self.weighted_choice.sample(rng);
        self.data.prev = self.children[index].1.next(rng);

        self.data.prev
    }

    fn data(&self) -> &RvData {
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
