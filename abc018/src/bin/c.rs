use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        mut s: [Chars; r]
    }

    if r < k || c < k {
        println!("0");
        return;
    }

    let mut deque = VecDeque::new();
    let mut seen = vec![vec![false; c]; r];

    for x in 0..r {
        for y in 0..c {
            if s[x][y] == 'x' {
                seen[x][y] = true;
                if x > 0 && s[x - 1][y] == 'o' {
                    deque.push_back(((x - 1, y), k - 2));
                }
                if x < r - 1 && s[x + 1][y] == 'o' {
                    deque.push_back(((x + 1, y), k - 2));
                }
                if y > 0 && s[x][y - 1] == 'o' {
                    deque.push_back(((x, y - 1), k - 2));
                }
                if y < c - 1 && s[x][y + 1] == 'o' {
                    deque.push_back(((x, y + 1), k - 2));
                }
            }
        }
    }

    while let Some(((x, y), i)) = deque.pop_front() {
        if seen[x][y] {
            continue;
        }

        seen[x][y] = true;
        s[x][y] = 'x';

        if i > 0 {
            if x > 0 && s[x - 1][y] == 'o' && !seen[x - 1][y] {
                deque.push_back(((x - 1, y), i - 1));
            }
            if x < r - 1 && s[x + 1][y] == 'o' && !seen[x + 1][y] {
                deque.push_back(((x + 1, y), i - 1));
            }
            if y > 0 && s[x][y - 1] == 'o' && !seen[x][y - 1] {
                deque.push_back(((x, y - 1), i - 1));
            }
            if y < c - 1 && s[x][y + 1] == 'o' && !seen[x][y + 1] {
                deque.push_back(((x, y + 1), i - 1));
            }
        }
    }

    let mut count = 0;
    for x in (k - 1)..=(r - k) {
        for y in (k - 1)..=(c - k) {
            if s[x][y] == 'o' {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
