extern crate reinforce;

use reinforce::Environment;


pub fn main() {
    println!("hello");
    let mut env = Environment::new();
    let num_of_learn = 10_000;
    let (win, lose, draw) = env.learn(num_of_learn);
    println!("random learn win: {}, lose: {}, drow: {} / {}", win, lose, draw, win + lose + draw);
}
