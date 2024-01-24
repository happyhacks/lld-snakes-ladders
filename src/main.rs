mod game;
use crate::game::board::SuperMove;
fn main() {
    let mut game = game::Game::new();
    game.add_player("player1".to_string());
    game.add_player("player2".to_string());
    game.board.add_supermove(5, SuperMove::Crocodile);
    game.board.add_supermove(10, SuperMove::Ladder(5));
    game.board.add_supermove(15, SuperMove::Snake(5));
    game.board.add_supermove(12, SuperMove::Mine(2));
    println!("{}", serde_yaml::to_string(&game).unwrap());
    for _ in 0..10 {
        game.go();
        println!("------------------------");
    }
    println!("{}", serde_yaml::to_string(&game).unwrap());
}
