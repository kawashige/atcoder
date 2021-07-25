use std::collections::HashSet;

use proconio::input;
use proconio::marker::Chars;

fn dfs(current: u64, k: usize, s: &Vec<Vec<char>>, seen: &mut HashSet<u64>, count: &mut usize) {
    if seen.contains(&current) {
        return;
    }
    seen.insert(current);

    if current.count_ones() as usize == k {
        *count += 1;
        return;
    }

    for l in 0..64 {
        if current & 1_u64 << l == 0 {
            continue;
        }
        let (i, j) = ((l / s.len() as u64) as usize, (l % s.len() as u64) as usize);

        for (r, c) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (x, y) = (i as i32 + r, j as i32 + c);
            if x < 0
                || y < 0
                || x >= s.len() as i32
                || y >= s[0].len() as i32
                || s[x as usize][y as usize] == '#'
                || current | 1 << (x as usize * s.len() + y as usize) == current
            {
                continue;
            };
            dfs(
                current | 1 << (x as usize * s.len() + y as usize),
                k,
                s,
                seen,
                count,
            );
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Chars; n]
    }

    let mut count = 0;
    let mut seen = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '.' {
                dfs(1 << (i * n + j), k, &s, &mut seen, &mut count);
            }
        }
    }

    println!("{}", count);
}
