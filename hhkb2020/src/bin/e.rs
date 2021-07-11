use proconio::input;
use proconio::marker::Chars;

fn mod_pow(n: usize, i: usize) -> usize {
    const M: usize = 1_000_000_007;
    if i == 0 {
        1
    } else if i % 2 == 0 {
        let r = mod_pow(n, i / 2);
        r * r % M
    } else {
        n * mod_pow(n, i - 1) % M
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    const M: i64 = 1_000_000_007;

    let mut total = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                total += 1;
            }
        }
    }

    let mut top = vec![vec![0; w]; h];
    for i in 1..h {
        for j in 0..w {
            if s[i][j] == '.' && s[i - 1][j] == '.' {
                top[i][j] += top[i - 1][j] + 1;
            }
        }
    }

    let mut bottom = vec![vec![0; w]; h];
    for i in (0..(h - 1)).rev() {
        for j in 0..w {
            if s[i][j] == '.' && s[i + 1][j] == '.' {
                bottom[i][j] += bottom[i + 1][j] + 1;
            }
        }
    }

    let mut left = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 1..w {
            if s[i][j] == '.' && s[i][j - 1] == '.' {
                left[i][j] += left[i][j - 1] + 1;
            }
        }
    }

    let mut right = vec![vec![0; w]; h];
    for i in 0..h {
        for j in (0..(w - 1)).rev() {
            if s[i][j] == '.' && s[i][j + 1] == '.' {
                right[i][j] += right[i][j + 1] + 1;
            }
        }
    }

    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                let n = 1 + top[i][j] + bottom[i][j] + left[i][j] + right[i][j];
                let mut c =
                    mod_pow(2, 1 + top[i][j] + bottom[i][j] + left[i][j] + right[i][j]) as i64 - 1;
                if c < 0 {
                    c += M;
                }
                c *= mod_pow(2, total - n) as i64;
                c %= M;
                count += c;
                count %= M;
            }
        }
    }

    println!("{}", count);
}
