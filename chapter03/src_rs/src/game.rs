pub const VALUE_O: i64 = 1;
pub const VALUE_X: i64 = 2;

const SIZE: usize  = 3;

const TOTAL_O: i64 = 1 * 1 * 1;
const TOTAL_X: i64 = 2 * 2 * 2;


pub type Board = [[i64; 3]; 3];


#[allow(dead_code)]
pub fn board_new() -> Board {
    return [
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
    ];
}

pub fn base_value() -> Board {
    let f = |x| {3i64.pow(x)};
    return [
        [f(8), f(7), f(6)],
        [f(5), f(4), f(3)],
        [f(2), f(1), f(0)],
    ]
}

pub fn to_lined_board(record: &Board) -> Board {
    let mut results = board_new();
    to_lined_board_row(&mut results, record);
    to_lined_board_column(&mut results, record);
    to_lined_board_upper_right_diagonal(&mut results, record);
    to_lined_board_lower_right_diagonal(&mut results, record);
    return results;
}

fn to_lined_board_row(results: &mut Board, record: &Board) {
    for i in 0..SIZE {
        match record[i].iter().fold(1, |sum, x| sum * x) {
            TOTAL_O => {
                for j in 0..SIZE {
                    results[i][j] = VALUE_O;
                }
            },
            TOTAL_X => {
                for j in 0..SIZE {
                    results[i][j] = VALUE_X;
                }
            },
            _ => {
            },
        }
    }
}

fn to_lined_board_column(results: &mut Board, record: &Board) {
    for j in 0..SIZE {
         match record[0][j] * record[1][j] * record[2][j] {
            TOTAL_O => {
                for i in 0..SIZE {
                     results[i][j] = VALUE_O;
                }
            },
            TOTAL_X => {
               for i in 0..SIZE {
                    results[i][j] = VALUE_X;
               }
            },
            _ => {
            },
         }
    }
}


fn to_lined_board_upper_right_diagonal(results: &mut Board, record: &Board) {
    match record[0][0] * record[1][1] * record[2][2] {
        TOTAL_O => {
            for i in 0..SIZE {
                results[i][i] = VALUE_O;
            }
        },
        TOTAL_X => {
            for i in 0..SIZE {
                results[i][i] = VALUE_X;
            }
        },
        _ => {
        },
    }
}

fn to_lined_board_lower_right_diagonal(results: &mut Board, record: &Board) {
    match record[2][0] * record[1][1] * record[0][2] {
        TOTAL_O => {
            for i in 0..SIZE {
                results[SIZE - 1 - i][i] = 1;
            }
        },
        TOTAL_X => {
            for i in 0..SIZE {
                results[SIZE - 1 - i][i] = 2;
            }
        },
        _ => {
        },
    }
}


pub fn count_line(xs: Board) -> i64 {
    return xs.iter()
        .fold(0, |sum, row|
              sum + row.iter().fold(0, |sum, v| sum + v));
}

pub fn is_first_player(turn: usize) -> bool {
    return turn % 2 == 1;
}

pub fn from_turn_to_player_value(turn: usize) -> i64 {
    return if turn % 2 == 1 { VALUE_O } else { VALUE_X };
}


#[cfg(test)]
mod tests {
    #[test]
    fn to_lined_board() {
        to_lined_board1();
        to_lined_board2();
        to_lined_board3();
    }

    fn to_lined_board1() {
        let b = [
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
        ];
        for i in &super::to_lined_board(&b) {
            for v in i {
                assert_eq!(*v, 0);
            }
        }
    }

    fn to_lined_board2() {
        let b = [
            [1, 1, 1],
            [0, 0, 0],
            [0, 0, 0],
        ];
        let xs = super::to_lined_board(&b);
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

    fn to_lined_board3() {
        let src = [
            [0, 0, 1],
            [0, 1, 0],
            [1, 0, 0],
        ];
        let xs = super::to_lined_board(&src);
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
