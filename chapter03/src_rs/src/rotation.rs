use super::{Board, Board1};


//対称反転
fn mirror_symmetry(source: Board, r: i32) -> Board {
    let num = 3;

    let mut results = Board1::new();

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
fn rotation_symmetry(source: Board, r: i32) -> Board {
    // let num = 3 - 1;
    let num = 3;

    let mut results = Board1::new();

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
fn point_symmetry(source: Board) -> Board {
    let mut results = Board1::new();

    results[0][0] = source[2][2]; //8
    results[0][1] = source[2][1]; //7
    results[0][2] = source[2][0]; //6

    results[1][0] = source[1][2]; //5
    results[1][1] = source[1][1]; //4
    results[1][2] = source[1][0]; //3

    results[2][0] = source[0][2]; //2
    results[2][1] = source[0][1]; //1
    results[2][2] = source[0][0]; //0

    return results;
}

//状態値を計算する関数
fn get_state_value(record: Board) -> u64 {
    let base_values = [
        [3u64.pow(8), 3u64.pow(7), 3u64.pow(6)],
        [3u64.pow(5), 3u64.pow(4), 3u64.pow(3)],
        [3u64.pow(2), 3u64.pow(1), 3u64.pow(0)],
    ];

    let mut v = 0;
    for i in 0..3 {
        for j in 0..3 {
            v += record[i][j] as u64 * base_values[i][j];
        }
    }
    return v;
}

//状態値が最小値となる対称性と状態値を計算
fn get_min_value(record: Board) -> (u64, i32, i32) {
    let mut min_v = 3u64.pow(10);
    let mut min_r = 0;
    let mut min_m = 0;

    for r in 0..4 {
        for m in 0..5 {
            let _record = rotation_symmetry(record, r);
            let __record = mirror_symmetry(_record, m);
            let v = get_state_value(__record);
            //より小さい状態値であれば更新
            if v < min_v {
                min_v = v;
                min_r = r;
                min_m = m;
            }
        }
    }
    // return {value :min_v, rotationSymmetry : min_r, mirrorSymmetry : min_m };
    return (min_v, min_r, min_m);
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
