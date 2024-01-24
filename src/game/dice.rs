use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Die {
    sides: u32,
}

impl Die {
    pub fn new(sides: u32) -> Self {
        Self { sides }
    }
    pub fn roll(&self) -> u32 {
        rand::thread_rng().gen_range(1..(self.sides + 1))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dice {
    count: u32,
    strategy: Strategy,
    die: Die,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum Strategy {
    Sum,
    #[default]
    Max,
    Min,
}

impl Strategy {
    pub fn apply(&self) -> fn(u32, u32) -> u32 {
        match self {
            Strategy::Sum => std::ops::Add::add,
            Strategy::Max => std::cmp::max,
            Strategy::Min => std::cmp::min,
        }
    }
}

impl Dice {
    pub fn new(count: u32, sides: u32) -> Self {
        Self {
            count,
            die: Die::new(sides),
            strategy: Strategy::default(),
        }
    }
    pub fn strategy(mut self, strategy: Strategy) -> Self {
        self.strategy = strategy;
        self
    }
    pub fn roll(&self) -> Option<u32> {
        (0..self.count)
            .map(|_| self.die.roll())
            .reduce(self.strategy.apply())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roll_single() {
        let d6 = Die::new(6);
        assert!(1 <= d6.roll() && d6.roll() <= 6);
    }

    #[test]
    fn roll_multiple() {
        let d6 = Dice::new(3, 6);
        assert!(1 <= d6.roll().unwrap() && d6.roll().unwrap() <= 6);
    }

    #[test]
    fn roll_sum() {
        let d6 = Dice::new(3, 6).strategy(Strategy::Sum);
        assert!(3 <= d6.roll().unwrap() && d6.roll().unwrap() <= 18);
    }

    #[test]
    fn roll_min() {
        let d6 = Dice::new(3, 6).strategy(Strategy::Min);
        assert!(1 <= d6.roll().unwrap() && d6.roll().unwrap() <= 6);
    }
}
