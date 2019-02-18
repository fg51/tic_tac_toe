    use super::game;

//対称反転
pub fn mirror_symmetry(source: game::Board, r: i64) -> game::Board {
    let num = 3;

    let mut results = game::board_new();

    match r {
        0 => {
            for (i, row) in source.iter().enumerate() {
                for (j, v) in row.iter().enumerate() {
                    results[i][j] = *v;
                }
            }
        },
        1 => {  // 横軸
            for i in 0..num {
                for j in 0..num {
                    results[i][j] = source[num - 1 - i][j];
                }
            }
        },
        2 => {  // 縦軸
            for i in 0..num {
                for j in 0..num {
                    results[i][j] = source[i][num - 1 - j];
                }
            }
        },
        3 => {  // 右上斜軸
            for i in 0..num {
                for j in 0..num {
                    results[i][j] = source[num - 1 - j][num - 1 - i];
                }
            }
        },
        4 => {  // 右下斜軸
            for i in 0..num {
                for j in 0..num {
                    results[i][j] = source[j][i];
                }
            }
        },
        _ => {
        },
    }
    return results;
}

//反時計回り
pub fn rotation_symmetry(source: game::Board, r: i64) -> game::Board {
    // let num = 3 - 1;
    let num = 3;

    let mut results = game::board_new();

    match r {
        0 => {
            for i in 0..num {
                for j in 0..num {
                    results[i][j] = source[i][j];
                }
            }
        },
        1 => { // 90 [deg]
            for i in 0..num {
                for j in 0..num {
                    results[i][j] = source[j][num - 1 - i];
                }
            }
        },
        2 => { // 180 [deg]
            for i in 0..num {
                for j in 0..num {
                    results[i][j] = source[num - 1 - i][num - 1- j];
                }
            }
        },
        3 => { // 270 [deg]
            for i in 0..num {
                for j in 0..num {
                    results[i][j] = source[num - 1 - j][i];
                }
            }
        },
        _ => {
        }
    }
    return results;
}

//点対称
pub fn point_symmetry(source: game::Board) -> game::Board {
    let mut result = game::board_new();

    result[0][0] = source[2][2]; //8
    result[0][1] = source[2][1]; //7
    result[0][2] = source[2][0]; //6

    result[1][0] = source[1][2]; //5
    result[1][1] = source[1][1]; //4
    result[1][2] = source[1][0]; //3

    result[2][0] = source[0][2]; //2
    result[2][1] = source[0][1]; //1
    result[2][2] = source[0][0]; //0

    return result;
}


#[cfg(test)]
mod tests {

    #[test]
    fn mirror_symmetry() {
        let b = [
            [1, 0, 0],
            [1, 0, 0],
            [1, 0, 0],
        ];

        let b = super::mirror_symmetry(b, 2);
        assert_eq!(b[0][0], 0);
        assert_eq!(b[0][1], 0);
        assert_eq!(b[0][2], 1);

        assert_eq!(b[1][0], 0);
        assert_eq!(b[1][1], 0);
        assert_eq!(b[1][2], 1);

        assert_eq!(b[2][0], 0);
        assert_eq!(b[2][1], 0);
        assert_eq!(b[2][2], 1);
    }
}
