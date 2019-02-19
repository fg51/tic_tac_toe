mod game;
mod symmetry;
mod agent;

use game::{from_turn_to_player_value, is_first_player};


pub struct Environment {
    player1: agent::Agent,
    player2: agent::Agent,
    records: Vec<Vec<game::Board>>,  // all patterns
    values: Vec<Vec<i64>>,  // all state values
    // base_value: game::Board,
    max_turn: usize,
}


impl Environment {
    pub fn new() -> Environment {
        let mut env = Environment {
            player1: agent::Agent::new(),  // 1st player
            player2: agent::Agent::new(),  // 2nd player
            records: vec![vec![game::board_new()]],
            values: vec![vec![0]],
            //base_value: game::base_value(),
            max_turn: 9,
        };
        for _ in 0..env.max_turn {
            env.values.push(vec![]);
            env.records.push(vec![]);
        }
        env.init();
        env.player1.set_use_boltzman();
        return env;
    }

    fn init(&mut self){
        for t in 1..(self.max_turn + 1) {
            // let t_move = 0;
            // let move_finish = 0;

            self.player1.add_blank_q_functions();
            self.player2.add_blank_q_functions();
            self.init_core(t as usize);
        }

        println!("ALL PATTERN (self.records)");
        for i in self.records.iter() {
            println!("{:?}", i);
        }
        println!("ALL STATE VALUES (self.values)");
        for i in self.values.iter() {
            println!("{:?}", i);
        }
        println!("player1: behavior evaluation function (self.player1.q_function)");
        println!("{:?}", self.player1.q_functions);
        println!("player2: behavior evaluation function (self.player2.q_function)");
        println!("{:?}", self.player2.q_functions);
    }

    fn init_core(&mut self, turn: usize) {
        let v = from_turn_to_player_value(turn);

        for i in 0..self.records[turn - 1].len() {
            // The number of patterns is T - t.
            for j in 0..(self.max_turn - turn + 1) {
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
                if is_first_player(turn) {
                    self.player1.q_functions[turn].push(0.0);
                } else {
                    self.player2.q_functions[turn].push(0.0);
                }

                self.values[turn].push(min_v);
                self.records[turn].push(
                    symmetry::mirror_symmetry(
                        symmetry::rotation_symmetry(record, min_r),
                        min_m ));
            }
        }
    }

    pub fn learn(&mut self, num: usize) -> (i64, i64, i64) {
        let mut win = 0;
        let mut lose = 0;
        let mut draw = 0;

        for _ in 1..(num + 1) {
            let mut record = game::board_new();

            //if parentID { self.createTable(record, null, parentID, null, true, true);

            let mut turn_history = vec![0];
            for turn in 1..(self.max_turn + 1) {
                let turn = turn as usize;

                //println!("next_move");
                let next_move = if is_first_player(turn as usize) {
                    self.player1.select_next_move(turn, &record, &self.values)
                } else {
                    self.player2.select_next_move(turn, &record, &self.values)
                };
                record[next_move.0][next_move.1] = from_turn_to_player_value(turn);

                //println!("to_min_state_values");
                let min_value_result = to_min_state_values(record);
                let min_v = min_value_result.0;

                let (te_num, has_min_value) = Self::search_te_number(min_v, &self.values[turn]);

                if has_min_value == false {
                    println!("ERROR 1: {} {}", turn, min_v);
                }

                // 過去の手番号配列に格納
                turn_history.push(te_num);

                //if( parentID ) this.createTable(record, null, parentID, null, true, true);

                //println!("count_line");
                let num_of_line = game::count_line(game::to_lined_board(&record));

                let reward = if num_of_line > 0 {1.0} else {0.0};

                //println!("update_q_function");
                if is_first_player(turn) {
                    self.player1.update_q_function(turn, te_num, reward);
                } else {
                    self.player2.update_q_function(turn, te_num, reward);
                }

                //println!("win lose");
                if num_of_line > 0 {
                    //println!("turn: {}", turn);
                    //println!("turn_history: {:?}", turn_history);
                    if is_first_player(turn) {
                        win += 1;
                        self.player2.give_penalty(turn, turn_history);
                    } else {
                        lose += 1;
                        self.player1.give_penalty(turn, turn_history);
                    }
                    break;
                }

                if turn == 9 {
                    draw += 1;
                }
                //println!("loop end");
            }
        }
        return (win, lose, draw);
    }

    fn search_te_number(min_v: i64, values: &Vec<i64>) -> (usize, bool) {
        for (i, v) in values.iter().enumerate() {
            if *v == min_v {
                return (i, true);
            }
        }
        return (0, false);
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
    let base_values = game::base_value();

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

