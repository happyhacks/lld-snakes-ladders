use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    name: String,
    pos: Position,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self { name, pos: Position{ pos: 0, state: MoveState::Normal} }
    }
    pub fn get_position(&self) -> Position {
        self.pos
    }
    pub fn set_position(&mut self, position: Position) {
        self.pos = position;
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Position {
    pub pos : u32,
    pub state: MoveState,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum MoveState {
    Winner,
    OnMine(u32),
    Normal,
}

impl MoveState {
    pub fn is_winner(&self) -> bool {
        matches!(self, MoveState::Winner)
    }
    pub fn can_move(&self) -> bool {
        match self {
            MoveState::Winner => false,
            MoveState::Normal => true,
            MoveState::OnMine(0) => true,            
            MoveState::OnMine(_) => false,            
        }
    }
}