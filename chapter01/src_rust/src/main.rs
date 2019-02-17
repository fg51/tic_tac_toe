pub fn main() {
    for i in create_records() {
        println!("{}", show_board(i));
    }
}


fn create_records() -> Vec<[[i32; 3]; 3]> {
    return vec![
        [[0, 0, 0], [0, 1, 0], [0, 0, 0], ],
        [[0, 2, 0], [0, 1, 0], [0, 0, 0], ],
        [[0, 2, 0], [0, 1, 0], [1, 0, 0], ],
        [[0, 2, 2], [0, 1, 0], [1, 0, 0], ],
        [[1, 2, 2], [0, 1, 0], [1, 0, 0], ],
        [[1, 2, 2], [2, 1, 0], [1, 0, 0], ],
        [[1, 2, 2], [2, 1, 0], [1, 0, 1], ],
    ];
}

fn show_board(xs: [[i32; 3]; 3]) -> String {
    const SEPARATOR: &str = "+---+---+---+";
    return [
        xs.iter().map(| ys | [
            format!("\n{}", SEPARATOR),
            format!("| {} |", ys.iter().map(|i| format!("{}", from_int_to_char(*i)))
                .collect::<Vec<_>>().join(" | ")),
        ].join("\n")).collect(),

        format!("{}", SEPARATOR),
        format!("{}", ""),
    ].join("\n");
}


fn from_int_to_char(x: i32) -> char {
    match x {
        0 => ' ',
        1 => 'O',
        2 => 'X',
        _ => ' ',
    }
}
