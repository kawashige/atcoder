use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut black = 0;
    let mut seen = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                black += 1;
                seen[i][j] = true;
            }
        }
    }

    let mut deque = VecDeque::new();
    deque.push_back(((0, 0), 1));

    while let Some(((i, j), c)) = deque.pop_front() {
        if seen[i][j] {
            continue;
        }

        if (i, j) == (h - 1, w - 1) {
            println!("{}", h * w - black - c);
            return;
        }

        seen[i][j] = true;

        if i > 0 && !seen[i - 1][j] {
            deque.push_back(((i - 1, j), c + 1));
        }
        if j > 0 && !seen[i][j - 1] {
            deque.push_back(((i, j - 1), c + 1));
        }
        if i < h - 1 && !seen[i + 1][j] {
            deque.push_back(((i + 1, j), c + 1));
        }
        if j < w - 1 && !seen[i][j + 1] {
            deque.push_back(((i, j + 1), c + 1));
        }
    }

    println!("-1");
}
