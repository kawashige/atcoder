use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut board: [Chars; h]
    }

    let mut s = (0, 0);
    let mut g = (0, 0);

    let mut map = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            if board[i][j] == 'S' {
                s = (i, j);
            } else if board[i][j] == 'G' {
                g = (i, j);
            } else if board[i][j].is_ascii_lowercase() {
                (*map.entry(board[i][j]).or_insert(Vec::new())).push((i, j));
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((s, 0));
    while let Some(((i, j), t)) = queue.pop_front() {
        if (i, j) == g {
            println!("{}", t);
            return;
        }

        if board[i][j] == '#' {
            continue;
        }

        if i > 0 && board[i - 1][j] != '#' {
            queue.push_back(((i - 1, j), t + 1));
        }
        if j > 0 && board[i][j - 1] != '#' {
            queue.push_back(((i, j - 1), t + 1));
        }
        if i < h - 1 && board[i + 1][j] != '#' {
            queue.push_back(((i + 1, j), t + 1));
        }
        if j < w - 1 && board[i][j + 1] != '#' {
            queue.push_back(((i, j + 1), t + 1));
        }
        if let Some(v) = map.get(&board[i][j]) {
            for (x, y) in v {
                if (x, y) != (&i, &j) && board[*x][*y] != '#' {
                    queue.push_back(((*x, *y), t + 1));
                }
            }
            map.remove(&board[i][j]);
        }
        board[i][j] = '#';
    }

    println!("-1");
}
