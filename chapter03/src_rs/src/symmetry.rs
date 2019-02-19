use super::game;

pub fn mirror_symmetry(src: game::Board, r: i64) -> game::Board {
    let num = 3;

    let mut results = game::board_new();

    match r {
        0 => {
            for (i, row) in src.iter().enumerate() {
                for (j, v) in row.iter().enumerate() {
                    results[i][j] = *v;
                }
            }
        },
        1 => {  // row line
            for i in 0..num {
                for j in 0..num {
                    results[i][j] = src[num - 1 - i][j];
                }
            }
        },
        2 => {  // column line
            for i in 0..num {
                for j in 0..num {
                    results[i][j] = src[i][num - 1 - j];
                }
            }
        },
        3 => {  // upper right diagonal line
            for i in 0..num {
                for j in 0..num {
                    results[i][j] = src[num - 1 - j][num - 1 - i];
                }
            }
        },
        4 => {  // lower right diagonal line
            for i in 0..num {
                for j in 0..num {
                    results[i][j] = src[j][i];
                }
            }
        },
        _ => {
        },
    }
    return results;
}

// inverted rotation
pub fn rotation_symmetry(src: game::Board, r: i64) -> game::Board {
    // let num = 3 - 1;
    let num = 3;

    let mut results = game::board_new();

    match r {
        0 => {
            for i in 0..num {
                for j in 0..num {
                    results[i][j] = src[i][j];
                }
            }
        },
        1 => { // 90 [deg]
            for i in 0..num {
                for j in 0..num {
                    results[i][j] = src[j][num - 1 - i];
                }
            }
        },
        2 => { // 180 [deg]
            for i in 0..num {
                for j in 0..num {
                    results[i][j] = src[num - 1 - i][num - 1- j];
                }
            }
        },
        3 => { // 270 [deg]
            for i in 0..num {
                for j in 0..num {
                    results[i][j] = src[num - 1 - j][i];
                }
            }
        },
        _ => {
        }
    }
    return results;
}

pub fn point_symmetry(src: game::Board) -> game::Board {
    let mut result = game::board_new();

    result[0][0] = src[2][2]; //8
    result[0][1] = src[2][1]; //7
    result[0][2] = src[2][0]; //6

    result[1][0] = src[1][2]; //5
    result[1][1] = src[1][1]; //4
    result[1][2] = src[1][0]; //3

    result[2][0] = src[0][2]; //2
    result[2][1] = src[0][1]; //1
    result[2][2] = src[0][0]; //0

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
