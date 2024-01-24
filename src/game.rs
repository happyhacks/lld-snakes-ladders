pub mod board;
mod dice;
mod player;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub board: board::Board,
    players: Vec<player::Player>,
    dice: dice::Dice,
}

impl Game {
    pub fn new() -> Self {
        Self { board: board::Board::new(10, 10),players: vec![], dice: dice::Dice::new(1, 6) }
    }
    pub fn add_player(&mut self, name: String) {
        self.players.push(player::Player::new(name));
    }
    pub fn go(&mut self) {
        for player in self.players.iter_mut() {
            let m = self.dice.roll();
            println!("{:?} roll {}", player, m.unwrap_or(0));
            if let Some(m) = m {
                player.set_position(self.board.go(player.get_position(), m));
         }
        }
    }
}