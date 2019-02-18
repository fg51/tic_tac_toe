extern crate rand;
use super::Board;

struct Agent {
    // environment: Environment,
    q_function: Vec<Vec<f64>>,  // 行動評価関数
    eta: f64,  // 学習率
    gamma: f64,  // 割引率
    lose_penalty: f64,  // 負け時のペナルティ
    select_method: i32,  // 行動選択の方法（0:ランダム、1:Epsilon-Greedy法、2:ボルツマン法）
    epsilon: f64,  // 貪欲性（Epsilon-Greedy法）
    beta: f64, // ボルツマン因子の指数（ボルツマン法）
}

impl Agent {
    pub fn new() -> Agent {
        return Agent {
            // environment: environment,
            q_function: vec![vec![0.0]],
            eta: 0.1,
            gamma: 1.0,
            lose_penalty: -2.0,
            select_method: 0,
            epsilon: 0.5,  // 貪欲性（Epsilon-Greedy法）
            beta: 1.0, // ボルツマン因子の指数（ボルツマン法）
        }
    }
}


impl Agent {
    //次の手を選択する関数
    fn select_next_move(&self, t: i32, record: Board) -> (i32, i32) {
        //let te_num;
        let mut _i;
        let mut _j;
        let select_random = false;

        // ランダム以外
        if self.select_method > 0 {
            //場合によってはランダムで選択
            if self.select_method == 2
                || (self.select_method == 1 && self.epsilon > rand::random())
                {
                //Epsilon-Greedy法を用いて次の手を選択
                let result = self.select_next_move_use_epsilon(t, record);
                _i = result.0;
                _j = result.1;
                if self.select_method == 2 {
                    //ボルツマン法を用いて次の手を選択
                    let result = self.select_next_move_use_boltzman( t, record );
                    _i = result.0;
                    _j = result.1;
                }

            } else {
                select_random = true;
            }
        } else {
            select_random = true;
        }

        //ランダム選択を実行
        if select_random {

            //次の手を乱数で決定（左上からの通し番号）
            let te = ((10 - t) as f64 * rand::random::<f64>()).floor() as i32;

            let mut blank = 0;
            for i in 0..record.len() {
                for j in 0..record.len() {
                    if record[i][j] == 0 {
                        blank += 1;
                    }
                    if blank == (te + 1) {
                        _i = i as i32;
                        _j = j as i32;
                        break;
                    }
                }
            }
        }
        return (_i, _j);
    }


    //Epsilon-Greedy法を用いて次の手を選択
    fn select_next_move_use_epsilon(self, t: i32, record: Board) -> (i32, i32) {
        let max_r = -10000;
        let mut _i;
        let mut _j;

        //ボルツマン因子
        // self._boltzmanFactors = []; // TODO

        //次の手の中で最もQ値が高い手を探索
        for i in 0..record.len() {
            for j in 0..record.len() {
                if record[i][j] == 0 {
                    if t % 2 == 1 {
                        record[i][j] = 1;
                    }
                    if t % 2 == 0 {
                        record[i][j] = 2;
                    }

                    //状態値が最小値となる対称性と状態値を計算
                    let min_value_result = self.environment.get_min_value(record);
                    let min_v = min_value_result.value;

                    //元に戻す
                    record[i][j] = 0;

                    //手番号
                    let te_num = self.environment.values[t].indexOf( min_v );
                    if te_num == -1 {
                        println!("エラー0 {0} {1}", t, min_v);
                    }

                    //行動評価関数の値を取得する
                    let Q = self.q_function[t][te_num];
                    //ボルツマン因子の計算に利用するパラメータを格納
                    self._boltzmanFactors.push({ i:i, j:j, Q:Q });

                    if Q > max_r {
                        max_r = Q;
                        _i = i as i32;
                        _j = j as i32;
                    }
                }
            }
        }
        return (_i, _j);
    }

    //ボルツマン法を用いて次の手を選択
    fn select_next_move_use_boltzman(self, t: i32, record: Board) -> (i32, i32) {
        let mut _i, _j;

        //状態和（規格化因子）
        let state_sum = 0;
        for m in 0..self._boltzmanFactors {
            state_sum += Math.exp( self.beta * m.Q );
        }

        let random = rand::random();
        let int_probability = 0;

        for m in 0..self._boltzmanFactors.len() {
            int_probability += (self.beta * self._boltzmanFactors[m].Q ).exp() / state_sum;
            if random < int_probability {
                _i = self._boltzmanFactors[m].0;
                _j = self._boltzmanFactors[m].1;
                break;
            }
        }
        return (_i, _j);
    }

}


// 状態値が最小値となる対称性と状態値を計算
fn get_min_value (record: Board) {
        let mut min_v = 3.pow(10);
        let mut min_r = 0;
        let mut min_m = 0;
        for r in 0..3 + 1 {
            for m in 0..4 + 1 {
                let _record = rotation_symmetry(record, r );
                let __record = mirror_symmetry(_record, m );
                let v = get_state_value(__record);
                //より小さい状態値であれば更新
                if  v < min_v {
                        min_v = v;
                        min_r = r;
                        min_m = m;
                }
            }
        }
        return {value :min_v, rotationSymmetry : min_r, mirrorSymmetry : min_m };
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
