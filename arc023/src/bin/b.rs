use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        d: usize,
        a: [[usize; c]; r]
    }

    let target = d % 2;
    let mut seen = vec![vec![false; c]; r];
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0));

    let mut max = 0;

    while let Some(((i, j), count)) = queue.pop_front() {
        if seen[i][j] {
            continue;
        }
        seen[i][j] = true;

        if count % 2 == target {
            max = max.max(a[i][j]);
        }

        if count < d {
            if i < r - 1 {
                queue.push_back(((i + 1, j), count + 1));
            }
            if j < c - 1 {
                queue.push_back(((i, j + 1), count + 1));
            }
        }
    }

    println!("{}", max);
}
