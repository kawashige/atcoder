use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut heap = BinaryHeap::new();
    let mut seen = vec![vec![false; w]; h];
    heap.push((Reverse(0), (0, 0)));

    while let Some((Reverse(c), (i, j))) = heap.pop() {
        if seen[i][j] {
            continue;
        }

        if i == h - 1 && j == w - 1 {
            println!("{}", c);
            return;
        }

        seen[i][j] = true;

        for (d_x, d_y) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (x, y) = (i as i32 + d_x, j as i32 + d_y);
            if x < 0
                || y < 0
                || x > h as i32 - 1
                || y > w as i32 - 1
                || seen[x as usize][y as usize]
            {
                continue;
            }

            if s[x as usize][y as usize] == '.' {
                heap.push((Reverse(c), (x as usize, y as usize)));
            } else {
                for (d_x2, d_y2) in [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .iter()
                {
                    let (x2, y2) = (x + d_x2, y + d_y2);
                    if x2 < 0
                        || y2 < 0
                        || x2 > h as i32 - 1
                        || y2 > w as i32 - 1
                        || seen[x2 as usize][y2 as usize]
                    {
                        continue;
                    }
                    heap.push((Reverse(c + 1), (x2 as usize, y2 as usize)));
                }
            }
        }
    }
}
