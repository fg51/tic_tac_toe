#![allow(dead_code)]

// mod agent;
mod rotation;

type Board = [[u32; 3]; 3];

struct Board1 {}

impl Board1 {
    fn new() -> Board {
        return [
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
        ];
    }
}

pub struct Environment {
    // sente: Agent,  // 1st player
    // gote: Agent,  // 2nd player
    records: Vec<Vec<Board>>,  // 全状態の配置を格納する配列
    values: Vec<u32>,  // 全状態値を格納する配列
    base_value: [[u64; 3]; 3],  // 桁表
    turn: u32  // 手数
}

impl Environment {
    pub fn new() -> Environment {
        let b: Board = [
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
        ];
        let env = Environment {
            //sente: Agent::new(),  // 1st player
            //gote: Agent::new(),  // 2nd player
            records: vec![vec![b]],
            values: vec![0],
            base_value: [
                [3u64.pow(8), 3u64.pow(7), 3u64.pow(6)],
                [3u64.pow(5), 3u64.pow(4), 3u64.pow(3)],
                [3u64.pow(2), 3u64.pow(1), 3u64.pow(0)],
            ],
            turn: 9,
        };
        // env.init();
        return env;
    }

    //fn init (&self){
    //    // let records = self.records;
    //    // let values = self.values;

    //    // １手目からスタート
    //    // for( let t = 1; t <= this.T; t++ ){
    //    for t in 1..(self.turn + 1) {
    //        self.init_a(t as isize);
    //    }

    //    println!("全状態配置（this.records）");
    //    println!("{:?}", self.records);
    //    println!("全状態値（this.values）");
    //    println!("{:?}", self.values);
    //    println!("先手行動評価関数（this.sente.Qfunction）");
    //    // println!("{:?}", self.sente.q_function);
    //    println!("後手行動評価関数（this.gote.Qfunction）");
    //    // println!("{:?}", self.gote.q_function);
    //}

    //fn init_a(&self, t: isize) {
    //    // let t_move = 0;
    //    // let move_finish = 0;

    //    let mut b: Board = [
    //        [0, 0, 0],
    //        [0, 0, 0],
    //        [0, 0, 0],
    //    ];
    //    // self.records[0][0] = *b; // n手目のマス目を初期化

    //    //self.values.push(vec![]);  // n手目のマス目を初期化

    //    //// self.sente.Qfunction[ t ] = [];
    //    //// self.gote.Qfunction[ t ] = [];
    //    //self.init_b(t as usize);
    //}

    //fn init_b(&self, t: usize) {
    //    //１手前の状態から次の手を指す
    //    // for( let te = 0; te < records[ t-1 ].length; te++ ){
    //    for te in 0..self.records[t - 1].len() {
    //        //打てるパターンはT-t個
    //        // for(let k=0; k<= this.T-t; k++){
    //        for k in 0..(self.turn - t as u32 + 1) {
    //            let mut record: Board;  // 新しいマス目の配置を格納する配列を準備
    //            //１手前の配置をコピー
    //            // for( let i = 0; i < records[ t-1 ][ te ].length; i++){
    //            for i in 0..self.records[t - 1][te].len() {
    //                // for( let j = 0; j < records[ t-1 ][ te ][ 0 ].length;j++ ){
    //                for j in 0..self.records[t - 1][te][0].len() {
    //                    record[i][j] = self.records[ t-1 ][ te ][ i ][ j ];
    //                }
    //            }
    //            // ラインのチェック
    //            let line_results = self.check_line(&record);
    //        }
    //    }
    //}

}

fn check_line(record: &Board) -> Board {
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
fn check_line1(results: &mut Board, record: &Board) {
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
fn check_line2(results: &mut Board, record: &Board) {
    for j in 0.. 3 {
         if record[0][j] * record[1][j] * record[2][j] == 1 {
            for i in 0..3 {
                 results[i][j] = 1;
            }
         }
         if record[0][j] * record[1][j] * record[2][j] == 8 {
            for i in 0..3 {
                 results[i][j] = 2;
            }
         }
    }
}


// 右上斜め
fn check_line3(results: &mut Board, record: &Board) {
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
fn check_line4(results: &mut Board, record: &Board) {
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

#[cfg(test)]
mod tests {
    use super::Environment;

    #[test]
    fn environment_new() {
        let env = Environment::new();
        assert_eq!(env.turn, 9);
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
