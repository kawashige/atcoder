use std::collections::HashMap;

use proconio::input;

fn recurse(
    board: &mut Vec<Vec<usize>>,
    turn: usize,
    b: &Vec<Vec<i32>>,
    c: &Vec<Vec<i32>>,
    memo: &mut HashMap<String, (i32, i32)>,
) -> (i32, i32) {
    if turn == 9 {
        let (mut r1, mut r2) = (0, 0);
        for i in 0..2 {
            for j in 0..3 {
                if board[i][j] == board[i + 1][j] {
                    r1 += b[i][j];
                } else {
                    r2 += b[i][j];
                }
            }
        }
        for i in 0..3 {
            for j in 0..2 {
                if board[i][j] == board[i][j + 1] {
                    r1 += c[i][j];
                } else {
                    r2 += c[i][j];
                }
            }
        }
        return (r1, r2);
    }

    let key = board
        .iter()
        .flatten()
        .map(|x| x.to_string())
        .collect::<String>();

    if let Some(s) = memo.get(&key) {
        return *s;
    }

    let v = if turn % 2 == 0 { 1 } else { 2 };
    let mut x = if turn % 2 == 0 {
        (std::i32::MIN, std::i32::MIN)
    } else {
        (std::i32::MIN, std::i32::MIN)
    };
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] != 0 {
                continue;
            }
            board[i][j] = v;
            let r = recurse(board, turn + 1, b, c, memo);
            if turn % 2 == 0 {
                if x.0 < r.0 {
                    x = r;
                }
            } else {
                if x.1 < r.1 {
                    x = r;
                }
            }
            board[i][j] = 0;
        }
    }

    memo.insert(key, x);
    x
}

fn main() {
    input! {
        b: [[i32; 3]; 2],
        c: [[i32; 2]; 3]
    }

    let r = recurse(&mut vec![vec![0; 3]; 3], 0, &b, &c, &mut HashMap::new());

    println!("{}", r.0);
    println!("{}", r.1);
}
