use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [Chars; h]
    }

    let mut queue = VecDeque::new();

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                if i > 0 && a[i - 1][j] == '.' {
                    queue.push_back(((i - 1, j), 1));
                }
                if j > 0 && a[i][j - 1] == '.' {
                    queue.push_back(((i, j - 1), 1));
                }
                if i < h - 1 && a[i + 1][j] == '.' {
                    queue.push_back(((i + 1, j), 1));
                }
                if j < w - 1 && a[i][j + 1] == '.' {
                    queue.push_back(((i, j + 1), 1));
                }
            }
        }
    }

    let mut last = 0;
    while let Some(((i, j), c)) = queue.pop_front() {
        if a[i][j] == '#' {
            continue;
        }
        a[i][j] = '#';
        last = c;

        if i > 0 && a[i - 1][j] == '.' {
            queue.push_back(((i - 1, j), c + 1));
        }
        if j > 0 && a[i][j - 1] == '.' {
            queue.push_back(((i, j - 1), c + 1));
        }
        if i < h - 1 && a[i + 1][j] == '.' {
            queue.push_back(((i + 1, j), c + 1));
        }
        if j < w - 1 && a[i][j + 1] == '.' {
            queue.push_back(((i, j + 1), c + 1));
        }
    }

    println!("{}", last);
}
