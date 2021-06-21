use proconio::input;
use proconio::marker::Chars;

fn dfs(
    start: (usize, usize),
    current: (usize, usize),
    d: i32,
    c: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
    max: &mut i32,
) {
    if start == current && d > 0 {
        if d >= 3 {
            *max = std::cmp::max(*max, d);
        }
        return;
    }

    if seen[current.0][current.1] {
        return;
    }

    seen[current.0][current.1] = true;

    if current.0 > 0 && c[current.0 - 1][current.1] == '.' {
        dfs(start, (current.0 - 1, current.1), d + 1, c, seen, max);
    }
    if current.1 > 0 && c[current.0][current.1 - 1] == '.' {
        dfs(start, (current.0, current.1 - 1), d + 1, c, seen, max);
    }
    if current.0 < c.len() - 1 && c[current.0 + 1][current.1] == '.' {
        dfs(start, (current.0 + 1, current.1), d + 1, c, seen, max);
    }
    if current.1 < c[0].len() - 1 && c[current.0][current.1 + 1] == '.' {
        dfs(start, (current.0, current.1 + 1), d + 1, c, seen, max);
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        mut c: [Chars; h]
    }

    let mut changed = true;
    while changed {
        changed = false;

        for i in 0..h {
            for j in 0..w {
                let mut count = 0;
                if i > 0 && c[i - 1][j] == '.' {
                    count += 1;
                }
                if j > 0 && c[i][j - 1] == '.' {
                    count += 1;
                }
                if i < h - 1 && c[i + 1][j] == '.' {
                    count += 1;
                }
                if j < w - 1 && c[i][j + 1] == '.' {
                    count += 1;
                }
                if count < 2 {
                    c[i][j] = '#';
                }
            }
        }
    }

    let mut max = -1;
    for i in 0..h {
        for j in 0..w {
            let mut seen = vec![vec![false; w]; h];
            if c[i][j] == '.' {
                dfs((i, j), (i, j), 0, &c, &mut seen, &mut max);
            }
        }
    }

    println!("{}", max);
}
