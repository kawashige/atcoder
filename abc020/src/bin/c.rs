use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;
use proconio::marker::Chars;

fn is_ok(
    x: usize,
    t: usize,
    s: &Vec<Vec<char>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> bool {
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), start));
    let mut seen = vec![vec![false; s[0].len()]; s.len()];

    while let Some((Reverse(d), (r, c))) = heap.pop() {
        if (r, c) == goal {
            return d <= t;
        }

        if d > t {
            return false;
        }

        if seen[r][c] {
            continue;
        }
        seen[r][c] = true;

        for (i, j) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (k, l) = (r as i32 + i, c as i32 + j);
            if k < 0
                || l < 0
                || k >= s.len() as i32
                || l >= s[0].len() as i32
                || seen[k as usize][l as usize]
            {
                continue;
            }

            heap.push((
                Reverse(
                    d + if s[k as usize][l as usize] == '#' {
                        x
                    } else {
                        1
                    },
                ),
                (k as usize, l as usize),
            ));
        }
    }
    true
}

fn main() {
    input! {
        h: usize,
        w: usize,
        t: usize,
        s: [Chars; h]
    }
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                start = (i, j);
            } else if s[i][j] == 'G' {
                goal = (i, j);
            }
        }
    }

    let mut ok = 1;
    let mut ng = t + 1;

    while ok + 1 < ng {
        let mid = (ok + ng) / 2;
        if is_ok(mid, t, &s, start, goal) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
