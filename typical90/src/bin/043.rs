use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [usize; 2],
        t: [usize; 2],
        grid: [Chars; h]
    }

    let mut queue = VecDeque::new();
    let mut seen = vec![vec![vec![false; 2]; w]; h];

    let (s_h, s_w) = (s[0] - 1, s[1] - 1);
    let (t_h, t_w) = (t[0] - 1, t[1] - 1);

    if s_h > 0 && grid[s_h - 1][s_w] == '.' {
        queue.push_back(((s_h - 1, s_w), 0, 0));
    }
    if s_h < h - 1 && grid[s_h + 1][s_w] == '.' {
        queue.push_back(((s_h + 1, s_w), 0, 0));
    }
    if s_w > 0 && grid[s_h][s_w - 1] == '.' {
        queue.push_back(((s_h, s_w - 1), 0, 1));
    }
    if s_w < w - 1 && grid[s_h][s_w + 1] == '.' {
        queue.push_back(((s_h, s_w + 1), 0, 1));
    }

    while let Some(((i, j), c, d)) = queue.pop_front() {
        if seen[i][j][d] {
            continue;
        }

        if (i, j) == (t_h, t_w) {
            println!("{}", c);
            return;
        }

        seen[i][j][d] = true;

        if i > 0 && grid[i - 1][j] == '.' && !seen[i - 1][j][0] {
            if d == 0 {
                queue.push_front(((i - 1, j), c, 0));
            } else {
                queue.push_back(((i - 1, j), c + 1, 0));
            }
        }
        if i < h - 1 && grid[i + 1][j] == '.' && !seen[i + 1][j][0] {
            if d == 0 {
                queue.push_front(((i + 1, j), c, 0));
            } else {
                queue.push_back(((i + 1, j), c + 1, 0));
            }
        }
        if j > 0 && grid[i][j - 1] == '.' && !seen[i][j - 1][1] {
            if d == 1 {
                queue.push_front(((i, j - 1), c, 1));
            } else {
                queue.push_back(((i, j - 1), c + 1, 1));
            }
        }
        if j < w - 1 && grid[i][j + 1] == '.' && !seen[i][j + 1][1] {
            if d == 1 {
                queue.push_front(((i, j + 1), c, 1));
            } else {
                queue.push_back(((i, j + 1), c + 1, 1));
            }
        }
    }
}
