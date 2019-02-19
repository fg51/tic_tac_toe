extern crate rand;

use super::game;
use super::game::{VALUE_O, VALUE_X, Board};

enum Method {
    Random,
    EpsilonGreedy,
    Boltzman,
}

type Position = (usize, usize);

pub struct Agent {
    q_functions: Vec<Vec<f64>>,  // behavior evaluation function
    eta: f64,  // learning ratio
    gamma: f64,  // reduce ratio
    lose_penalty: f64,
    select_method: Method,
    epsilon: f64,  // greed for Epsilon-Greedy method
    beta: f64, // weight of boltzman factor for boltzman method

    boltzman_factors: Vec<(usize, usize, f64)>,
}

impl Agent {
    pub fn new() -> Agent {
        return Agent {
            q_functions: vec![vec![0.0]],
            eta: 0.1,
            gamma: 1.0,
            lose_penalty: -2.0,
            select_method: Method::Random,
            epsilon: 0.5,
            beta: 1.0,
            boltzman_factors: vec![],
        }
    }

    pub fn select_next_move(&mut self, turn: i32, record: &Board, state_values: Vec<Vec<i64>>) -> (usize, usize) {

        match self.select_method {
            Method::EpsilonGreedy if self.epsilon > rand::random() => {
                let position = self.select_next_move_use_epsilon(turn, &record, &state_values);
                return (position.0, position.1);
            },
            Method::Boltzman => {
                let _ = self.select_next_move_use_epsilon(turn, &record, &state_values);
                let position = self.select_next_move_use_boltzman(turn, &record);
                return (position.0, position.1);
            }
            _ => {
                let position = select_next_move_use_random(turn as usize, &record);
                return (position.0, position.1);
            }
        }
    }


    //Epsilon-Greedy method
    fn select_next_move_use_epsilon(&mut self, t: i32, record: &Board, state_values: &Vec<Vec<i64>>) -> Position {
        let t = t as usize;
        let mut max_r = -10000_f64;
        let mut row = 0;
        let mut col = 0;

        self.boltzman_factors = vec![];

        let mut xss: Board = game::board_new();
        xss[0][0] = record[0][0];
        xss[0][1] = record[0][1];
        xss[0][2] = record[0][2];
        xss[1][0] = record[1][0];
        xss[1][1] = record[1][1];
        xss[1][2] = record[1][2];
        xss[2][0] = record[2][0];
        xss[2][1] = record[2][1];
        xss[2][2] = record[2][2];

        // search the max_Q select
        for i in 0..record.len() {
            for j in 0..record.len() {
                if record[i][j] == 0 {
                    xss[i][j] = if game::is_first_player(t) {VALUE_O} else {VALUE_X};

                    let min_value_result = super::to_min_state_values(xss);
                    let min_v = min_value_result.0;

                    // reset
                    xss[i][j] = 0;

                    let mut has_value = false;
                    let mut index = 0;
                    for (i, v) in state_values[t].iter().enumerate() {
                        if *v == min_v {
                            has_value = true;
                            index = i;
                            break;
                        }
                    }
                    if has_value == false {
                        println!("ERROR 0 {0} {1}", t, min_v);
                    }

                    let q = self.q_functions[t][index];
                    self.boltzman_factors.push((i, j, q));

                    if q > max_r {
                        max_r = q;
                        row = i as usize;
                        col = j as usize;
                    }
                }
            }
        }
        return (row, col);
    }

    fn select_next_move_use_boltzman(&mut self, _: i32, _: &Board) -> Position {
        let mut _i: usize = 0;
        let mut _j: usize = 0;

        //状態和 (規格化因子)
        let state_sum = self.boltzman_factors.iter()
            .fold(0.0, |sum, m| sum + (self.beta * m.2).exp());

        let random = rand::random::<f64>();
        let mut int_probability = 0.0;

        for m in 0..self.boltzman_factors.len() {
            int_probability += (self.beta * self.boltzman_factors[m].2 ).exp() / state_sum;
            if random < int_probability {
                _i = self.boltzman_factors[m].0;
                _j = self.boltzman_factors[m].1;
                break;
            }
        }
        return (_i, _j);
    }

    pub fn update_q_function(&self, t: usize, te_num: usize, r: f64){
        let mut maxR = -1000.0;
        if t >= 8 || r > 0.0 {
            maxR = 0.0;
        } else {
            for i in 0..self.q_functions[t + 2].len() {
                if self.q_functions[t + 2][i] > maxR {
                    maxR = self.q_functions[t + 2][i];
                }
            }
        }
        let dQ = self.q_functions[t][te_num] - (r + self.gamma * maxR);
        self.q_functions[t][te_num] = self.q_functions[t][te_num] - self.eta * dQ;
    }

    pub fn give_penalty(&mut self, turn: usize , te_nums: Vec<usize>) {
        let mut m = 1;
        // TODO
        //for( let t = T - 1 ; t >= 1; t -= 2  ){
        for i in 1..turn - 1 {
            self.q_functions[i][te_nums[i]] += self.lose_penalty * self.eta * (self.gamma.powi(m));
            m += 1;
        }
    }
}

fn select_next_move_use_random(turn: usize, record: &Board) -> Position {
    let te = ((10 - turn) as f64 * rand::random::<f64>()).floor() as usize;

    let mut _i = 0;
    let mut _j = 0;
    let mut blank = 0;
    for i in 0..record.len() {
        for j in 0..record.len() {
            if record[i][j] == 0 {
                blank += 1;
            }
            if blank == (te + 1) {
                _i = i as usize;
                _j = j as usize;
                break;
            }
        }
    }
    return (_i, _j);
}


#[cfg(test)]
mod tests {
    use super::Agent;

    #[test]
    fn agent_new() {
        let a = Agent::new();
        assert_eq!(a.eta, 0.1);
    }
}
