use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut max = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            let mut queue = VecDeque::new();
            queue.push_back(((i, j), 0));
            let mut seen = vec![vec![false; w]; h];
            while let Some(((x, y), c)) = queue.pop_front() {
                if seen[x][y] || s[x][y] == '#' {
                    continue;
                }
                seen[x][y] = true;
                max = std::cmp::max(max, c);
                if x > 0 {
                    queue.push_back(((x - 1, y), c + 1));
                }
                if x < h - 1 {
                    queue.push_back(((x + 1, y), c + 1));
                }
                if y > 0 {
                    queue.push_back(((x, y - 1), c + 1));
                }
                if y < w - 1 {
                    queue.push_back(((x, y + 1), c + 1));
                }
            }
        }
    }

    println!("{}", max);
}
