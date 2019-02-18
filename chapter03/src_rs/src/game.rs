pub const VALUE_O:i64 = 1;
pub const VALUE_X:i64 = 2;

pub type Board = [[i64; 3]; 3];

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

pub fn check_line(record: Board) -> Board {
    let mut results: Board = [
        [0, 0, 0, ],
        [0, 0, 0, ],
        [0, 0, 0, ],
    ];

    check_line1(&mut results, &record); // 横列
    check_line2(&mut results, &record); // 縦列
    check_line3(&mut results, &record); // 右上斜め
    check_line4(&mut results, &record); // 右下斜め
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
