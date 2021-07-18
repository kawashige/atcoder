use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut r = 0;
    let mut seen = vec![vec![false; w]; h];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' || seen[i][j] {
                continue;
            }
            let mut queue = VecDeque::new();
            let mut count = vec![0; 2];
            queue.push_back(((i, j), s[i][j]));
            while let Some(((k, l), c)) = queue.pop_front() {
                if seen[k][l] {
                    continue;
                }

                seen[k][l] = true;

                if s[k][l] == '#' {
                    count[0] += 1;
                } else {
                    count[1] += 1;
                }

                if k > 0 && s[k - 1][l] != c && !seen[k - 1][l] {
                    queue.push_back(((k - 1, l), s[k - 1][l]));
                }
                if k < h - 1 && s[k + 1][l] != c && !seen[k + 1][l] {
                    queue.push_back(((k + 1, l), s[k + 1][l]));
                }
                if l > 0 && s[k][l - 1] != c && !seen[k][l - 1] {
                    queue.push_back(((k, l - 1), s[k][l - 1]));
                }
                if l < w - 1 && s[k][l + 1] != c && !seen[k][l + 1] {
                    queue.push_back(((k, l + 1), s[k][l + 1]));
                }
            }
            r += count[0] as u64 * count[1] as u64;
        }
    }

    println!("{}", r);
}
