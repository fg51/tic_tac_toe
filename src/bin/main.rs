extern crate tic_tac_toe;

use tic_tac_toe::Game;

fn main() {
    println!("Hello, world");
    let mut g = Game::new();
    g.play_game();
}
