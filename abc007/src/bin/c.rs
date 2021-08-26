use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        r: usize,
        c: usize,
        st: (usize, usize),
        g: (usize, usize),
        s: [Chars; r]
    }

    let mut deque = VecDeque::new();
    deque.push_back(((st.0 - 1, st.1 - 1), 0));
    let mut seen = vec![vec![false; c]; r];

    while let Some(((i, j), count)) = deque.pop_front() {
        if (i + 1, j + 1) == g {
            println!("{}", count);
            return;
        }

        if seen[i][j] {
            continue;
        }
        seen[i][j] = true;

        for (x, y) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (x2, y2) = (i as i32 + x, j as i32 + y);
            if x2 < 0
                || x2 >= r as i32
                || y2 < 0
                || y2 >= c as i32
                || seen[x2 as usize][y2 as usize]
                || s[x2 as usize][y2 as usize] == '#'
            {
                continue;
            }
            deque.push_back(((x2 as usize, y2 as usize), count + 1));
        }
    }
}
