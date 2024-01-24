use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::player::{MoveState, Position};

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    width: u32,
    height: u32,
    supermoves: HashMap<u32, SuperMove>,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            supermoves: HashMap::new(),
        }
    }
    pub fn winner(&self, pos: u32) -> bool {
        pos == self.height * self.width
    }
    pub fn add_supermove(&mut self, pos: u32, supermove: SuperMove) {
        self.supermoves.insert(pos, supermove);
    }
    pub fn go(&self, pos: Position, m: u32) -> Position {
        if !pos.state.can_move() {
            if let MoveState::OnMine(val) = pos.state {
                return Position {
                    pos: pos.pos,
                    state: MoveState::OnMine(val - 1),
                };
            }
            return pos;
        }
        let new_pos = pos.pos + m;
        if new_pos > self.height * self.width {
            // don't move if cross winner
            return pos;
        } else if new_pos == self.height * self.width {
            return Position {
                pos: new_pos,
                state: MoveState::Winner,
            };
        }
        let mut ret = Position {
            pos: new_pos,
            state: MoveState::Normal,
        };
        if let Some(supermove) = self.supermoves.get(&new_pos) {
            ret.pos = supermove.apply(new_pos);
            if let SuperMove::Mine(val) = supermove {
                ret.state = MoveState::OnMine(*val);
            }
        }
        ret
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SuperMove {
    Snake(u32),
    Ladder(u32),
    Mine(u32),
    Crocodile,
    GoHome,
}

impl SuperMove {
    pub fn apply(self, pos: u32) -> u32 {
        match self {
            SuperMove::Snake(val) => pos.saturating_sub(val),
            SuperMove::Ladder(val) => pos + val,
            SuperMove::Crocodile => pos.saturating_sub(5),
            SuperMove::GoHome => 0,
            _ => pos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn move_snake() {
        let snake = SuperMove::Snake(5);
        assert_eq!(snake.apply(4), 0);
    }
}
