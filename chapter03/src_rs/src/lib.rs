#![allow(dead_code)]

mod symmetry;

const VALUE_O:i64 = 1;
const VALUE_X:i64 = 2;


mod game {
    pub type Board = [[i64; 3]; 3];

    pub fn board_new() -> Board {
        return [
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
        ];
    }

    pub fn base_value() -> Board {
        return [
            [3i64.pow(8), 3i64.pow(7), 3i64.pow(6)],
            [3i64.pow(5), 3i64.pow(4), 3i64.pow(3)],
            [3i64.pow(2), 3i64.pow(1), 3i64.pow(0)],
        ]
    }
}


pub struct Environment {
    // sente: Agent,  // 1st player
    // gote: Agent,  // 2nd player
    records: Vec<Vec<game::Board>>,  // 全状態の配置を格納する配列
    values: Vec<Vec<i64>>,  // 全状態値を格納する配列
    base_value: game::Board,  // 桁表
    max_turn: u32  // 手数
}


impl Environment {
    pub fn new() -> Environment {
        let mut env = Environment {
            //sente: Agent::new(),  // 1st player
            //gote: Agent::new(),  // 2nd player
            records: vec![vec![game::board_new()]],
            values: vec![vec![0]],
            base_value: game::base_value(),
            max_turn: 9,
        };
        env.init();
        return env;
    }

    fn init (&mut self){
        // let records = self.records;
        // let values = self.values;

        // １手目からスタート
        for t in 1..(self.max_turn + 1) {
            self.init_a(t as isize);
        }

        println!("全状態配置（self.records）");
        for i in self.records.iter() {
            println!("{:?}", i);
        }
        println!("全状態値（self.values）");
        println!("{:?}", self.values);
        // println!("先手行動評価関数（this.sente.Qfunction）");
        // println!("{:?}", self.sente.q_function);
        // println!("後手行動評価関数（this.gote.Qfunction）");
        // println!("{:?}", self.gote.q_function);
    }

    fn init_a(&mut self, t: isize) {
        // let t_move = 0;
        // let move_finish = 0;

        // self.sente.Qfunction[ t ] = [];
        // self.gote.Qfunction[ t ] = [];
        self.init_b(t as usize);
    }

    fn init_b(&mut self, t: usize) {
        for te in 0..self.records[t - 1].len() {
            //打てるパターンはT-t個
            for k in 0..(self.max_turn - t as u32 + 1) {
                let mut record = game::board_new();
                for i in 0..self.records[t - 1][te].len() {
                    for j in 0..self.records[t - 1][te][0].len() {
                        record[i][j] = self.records[t - 1][te][ i ][ j ];
                    }
                }

                // ラインのチェック
                let line_results = check_line(&record);

                // ライン数
                let mut num_of_line = 0;
                for row in line_results.iter() {
                    for v in row.iter() {
                        if *v != 0 {
                            num_of_line += 1;
                        }
                    }
                }

                if num_of_line > 0 {
                    continue;
                }

                // 未配置のマス目に手を指す
                let mut blank = 0;
                'block:
                    for i in 0..record.len() {
                        for j in 0..record[i].len() {
                            if self.records[t - 1][te][i][j] == 0 {
                                blank += 1;
                            }
                            if blank == k + 1 {
                                if t % 2 == 1 {
                                    record[i][j] = 1; //先手
                                }
                                if t % 2 == 0 {
                                    record[i][j] = 2; //後手
                                }
                                break 'block;
                            }
                        }
                    }

                //状態値が最小値となる対称性と状態値を計算
                let min_value_result = get_min_value(record);
                let min_v = min_value_result.0;
                let min_r = min_value_result.1;
                let min_m = min_value_result.2;

                //状態値の最小値の出現が初めての場合
                if self.values.len() <= t {
                        //Q値の初期値を与える
                        //if(t%2==1) this.sente.Qfunction[ t ].push(0);
                        //if(t%2==0) this.gote.Qfunction[ t ].push(0);


                        //状態値として追加
                        self.values.push(vec![min_v]);
                        //配置として追加
                        let _record = symmetry::rotation_symmetry( record, min_r );
                        let __record = symmetry::mirror_symmetry( _record, min_m );
                        self.records.push(vec![__record]);
                } else {
                    let mut is_1st = true;
                    for i in self.values[t].iter() {
                        if *i == min_v {
                            is_1st = false;
                        }
                    }

                    if is_1st {
                        //Q値の初期値を与える
                        //if(t%2==1) this.sente.Qfunction[ t ].push(0);
                        //if(t%2==0) this.gote.Qfunction[ t ].push(0);


                        //状態値として追加
                        self.values[t].push(min_v);
                        //配置として追加
                        let _record = symmetry::rotation_symmetry( record, min_r );
                        let __record = symmetry::mirror_symmetry( _record, min_m );
                        self.records[t].push(__record);
                    }
                }
            }
        }
    }
}

fn check_line(record: &game::Board) -> game::Board {
    let mut results = [
        [0, 0, 0, ],
        [0, 0, 0, ],
        [0, 0, 0, ],
    ];

    check_line1(&mut results, record); // 横列
    check_line2(&mut results, record); // 縦列
    check_line3(&mut results, record); // 右上斜め
    check_line4(&mut results, record); // 右下斜め
    return results;
}

// 横列
fn check_line1(results: &mut game::Board, record: &game::Board) {
    for i in 0..3 {
        if record[i][0] * record[i][1] * record[i][2] == 1 {
            for j in 0..3 {
                results[i][j] = 1;
            }
        }
        if record[i][0] * record[i][1] * record[i][2] == 8 {
            for j in 0..3 {
                results[i][j] = 2;
            }
        }
    }
}

// 縦列
fn check_line2(results: &mut game::Board, record: &game::Board) {
    for j in 0.. 3 {
         if record[0][j] * record[1][j] * record[2][j] == 1 {
            for i in 0..3 {
                 results[i][j] = VALUE_O;
            }
         }
         if record[0][j] * record[1][j] * record[2][j] == 8 {
            for i in 0..3 {
                 results[i][j] = VALUE_X;
            }
         }
    }
}


// 右上斜め
fn check_line3(results: &mut game::Board, record: &game::Board) {
    if record[0][0] * record[1][1] * record[2][2] == 1  {
        for i in 0..3 {
            results[i][i] = 1;
        }
    }
    if record[0][0] * record[1][1] * record[2][2] == 8 {
        for i in 0..3 {
            results[i][i] = 2;
        }
    }
}

// 右下斜め
fn check_line4(results: &mut game::Board, record: &game::Board) {
    if record[2][0] * record[1][1] * record[0][2] == 1 {
        for i in 0..3 {
            results[3 - 1 - i][i] = 1;
        }
    }
    if record[2][0] * record[1][1] * record[0][2] == 8 {
        for i in 0..3 {
            results[3 - 1 - i][i] = 2;
        }
    }
}


fn cal_num_of_line(xs: [[i64; 3]; 3]) -> i64 {
    return xs.iter()
        .fold(0, |sum, row|
              sum + row.iter().fold(0, |sum, v| sum + v));
}


#[cfg(test)]
mod tests {
    use super::Environment;

    #[test]
    fn environment_new() {
        let env = Environment::new();
        assert_eq!(env.max_turn, 9);
    }

    #[test]
    fn check_line() {
        check_line1();
        check_line2();
        check_line3();
    }

    fn check_line1() {
        let b = [
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
        ];
        for i in &super::check_line(&b) {
            for v in i {
                assert_eq!(*v, 0);
            }
        }
    }

    fn check_line2() {
        let b = [
            [1, 1, 1],
            [0, 0, 0],
            [0, 0, 0],
        ];
        let xs = super::check_line(&b);
        for v in &xs[0] {
            assert_eq!(*v, 1);
        }
        for v in &xs[1] {
            assert_eq!(*v, 0);
        }
        for v in &xs[2] {
            assert_eq!(*v, 0);
        }
    }

    fn check_line3() {
        let b = [
            [0, 0, 1],
            [0, 1, 0],
            [1, 0, 0],
        ];
        let xs = super::check_line(&b);
        for (i, row) in xs.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                match i {
                    0 => match j {
                        2 => assert_eq!(*v, 1),
                        _ => assert_eq!(*v, 0),
                    },
                    1 => match j {
                        1 => assert_eq!(*v, 1),
                        _ => assert_eq!(*v, 0),
                    }
                    2 => match j {
                        0 => assert_eq!(*v, 1),
                        _ => assert_eq!(*v, 0),
                    }
                    _ => assert_eq!(*v, 0),
                }
            }
        }
    }
}



//状態値を計算する関数
fn get_state_value(record: game::Board) -> i64 {
    let base_values = [
        [3i64.pow(8), 3i64.pow(7), 3i64.pow(6)],
        [3i64.pow(5), 3i64.pow(4), 3i64.pow(3)],
        [3i64.pow(2), 3i64.pow(1), 3i64.pow(0)],
    ];

    let mut v = 0;
    for i in 0..3 {
        for j in 0..3 {
            v += record[i][j] * base_values[i][j];
        }
    }
    return v;
}

//状態値が最小値となる対称性と状態値を計算
fn get_min_value(record: game::Board) -> (i64, i64, i64) {
    let mut min_v = 3i64.pow(10);
    let mut min_r = 0;
    let mut min_m = 0;

    for r in 0..4 {
        for m in 0..5 {
            let _record = symmetry::rotation_symmetry(record, r as i64);
            let __record = symmetry::mirror_symmetry(_record, m);
            let v = get_state_value(__record);
            //より小さい状態値であれば更新
            if v < min_v {
                min_v = v;
                min_r = r;
                min_m = m;
            }
        }
    }
    return (min_v, min_r as i64, min_m as i64);
}
