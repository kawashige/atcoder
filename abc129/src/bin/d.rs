use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut up = vec![vec![0; w]; h];
    let mut down = vec![vec![0; w]; h];
    let mut left = vec![vec![0; w]; h];
    let mut right = vec![vec![0; w]; h];

    for i in 1..h {
        for j in 0..w {
            if s[i][j] == '.' && s[i - 1][j] == '.' {
                up[i][j] = up[i - 1][j] + 1;
            }
        }
    }

    for i in 0..h {
        for j in 1..w {
            if s[i][j] == '.' && s[i][j - 1] == '.' {
                left[i][j] = left[i][j - 1] + 1;
            }
        }
    }

    for i in (0..(h - 1)).rev() {
        for j in 1..w {
            if s[i][j] == '.' && s[i + 1][j] == '.' {
                down[i][j] = down[i + 1][j] + 1;
            }
        }
    }

    for i in 0..h {
        for j in (0..(w - 1)).rev() {
            if s[i][j] == '.' && s[i][j + 1] == '.' {
                right[i][j] = right[i][j + 1] + 1;
            }
        }
    }

    let mut max = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            max = std::cmp::max(max, 1 + up[i][j] + down[i][j] + left[i][j] + right[i][j]);
        }
    }
    println!("{}", max);
}
