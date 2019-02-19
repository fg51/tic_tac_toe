#![allow(dead_code)]

mod game;
mod symmetry;

use crate::game::{VALUE_O, VALUE_X};


pub struct Environment {
    // player1: Agent,  // 1st player
    // player2: Agent,  // 2nd player
    records: Vec<Vec<game::Board>>,  // all patterns
    values: Vec<Vec<i64>>,  // all state values
    base_value: game::Board,
    max_turn: u32,
}


impl Environment {
    pub fn new() -> Environment {
        let mut env = Environment {
            //player1: Agent::new(),  // 1st player
            //player2: Agent::new(),  // 2nd player
            records: vec![vec![game::board_new()]],
            values: vec![vec![0]],
            base_value: game::base_value(),
            max_turn: 9,
        };
        for _ in 0..env.max_turn {
            env.values.push(vec![]);
            env.records.push(vec![]);
        }
        env.init();
        return env;
    }

    fn init (&mut self){
        for t in 1..(self.max_turn + 1) {
            self.init_a(t as usize);
        }
        println!("ALL PATTERN (self.records)");
        for i in self.records.iter() {
            println!("{:?}", i);
        }
        println!("ALL STATE VALUES (self.values)");
        for i in self.values.iter() {
            println!("{:?}", i);
        }
        // println!("player1: behavior evaluation function (self.player1.q_function)");
        // println!("{:?}", self.player1.q_functions);
        // println!("player2: behavior evaluation function (self.player2.q_function)");
        // println!("{:?}", self.player2.q_functions);
    }

    fn init_a(&mut self, turn: usize) {
        // let t_move = 0;
        // let move_finish = 0;

        // self.player1.q_functions[turn] = [];
        // self.player2.q_functions[turn] = [];
        self.init_b(turn);
    }

    fn init_b(&mut self, turn: usize) {
        // 1: 1st player, 2: 2nd player
        let v = if game::is_first_player(turn) { VALUE_O } else { VALUE_X };

        for i in 0..self.records[turn - 1].len() {
            // The number of patterns is T - t.
            for j in 0..(self.max_turn - turn as u32 + 1) {
                let mut record = self.records[turn - 1][i];
                let num_of_line = game::count_line(game::to_lined_board(&record));
                if num_of_line > 0 {
                    continue;
                }

                select_blank_position(
                    &mut record,
                    &self.records[turn - 1][i],
                    j as i64 + 1,
                    v);

                let (min_v, min_r, min_m) = to_min_state_values(record);

                if is_1st_min_state_value(min_v, &self.values[turn])  == false {
                    continue;
                }
                // init Q values
                //if is_first_player(turn) {
                //    self.player1.q_functions[turn].push(0);
                //} else {
                //    self.gote.q_functions[turn].push(0);
                //}

                self.values[turn].push(min_v);
                self.records[turn].push(
                    symmetry::mirror_symmetry(
                        symmetry::rotation_symmetry(record, min_r),
                        min_m ));
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::Environment;

    #[test]
    fn environment_new() {
        let env = Environment::new();
        assert_eq!(env.max_turn, 9);
    }
}


fn to_state_value(record: game::Board) -> i64 {
    let b = 3i64;
    let base_values = [
        [b.pow(8), b.pow(7), b.pow(6)],
        [b.pow(5), b.pow(4), b.pow(3)],
        [b.pow(2), b.pow(1), b.pow(0)],
    ];

    let mut v = 0;
    for (irow, jrow) in base_values.iter().zip(record.iter()) {
        for (i, j) in irow.iter().zip(jrow) {
            v += i * j;
        }
    }
    return v;
}

fn to_min_state_values(record: game::Board) -> (i64, i64, i64) {
    let mut min_v = 3i64.pow(10);
    let mut min_r = 0;
    let mut min_m = 0;

    for r in 0..4 {
        for m in 0..5 {
            let v = to_state_value(symmetry::mirror_symmetry(
                symmetry::rotation_symmetry(record, r),
                m));
            if v < min_v {
                min_v = v;
                min_r = r;
                min_m = m;
            }
        }
    }
    return (min_v, min_r, min_m);
}

fn select_blank_position(
    record: &mut game::Board,
    old_record: &game::Board,
    target_blank: i64,
    value: i64
) {
    let mut blank = 0;
    'block: for i in 0..record.len() {
        for j in 0..record[i].len() {
            if old_record[i][j] == 0 {
                blank += 1;
            }
            if blank == target_blank {
                record[i][j] = value;
                break 'block;
            }
        }
    }
}

fn is_1st_min_state_value(min_v: i64, values: &Vec<i64>) -> bool{
    for i in values.iter() {
        if *i == min_v {
            return false;
        }
    }
    return true;
}
