use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let m = 1_000_000_007;

    let mut dp_up = vec![vec![0; w]; h];
    let mut dp_left = vec![vec![0; w]; h];
    let mut dp_diagonal = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            if i == 0 && j == 0 {
                dp_up[0][0] = 1;
                dp_left[0][0] = 1;
                dp_diagonal[0][0] = 1;
            } else {
                let mut dp = 0;
                if i > 0 && s[i - 1][j] == '.' {
                    dp += dp_up[i - 1][j];
                    dp_up[i][j] = dp_up[i - 1][j];
                }
                if j > 0 && s[i][j - 1] == '.' {
                    dp = (dp + dp_left[i][j - 1]) % m;
                    dp_left[i][j] = dp_left[i][j - 1];
                }
                if i > 0 && j > 0 && s[i - 1][j - 1] == '.' {
                    dp = (dp + dp_diagonal[i - 1][j - 1]) % m;
                    dp_diagonal[i][j] = dp_diagonal[i - 1][j - 1];
                }
                dp_up[i][j] = (dp_up[i][j] + dp) % m;
                dp_left[i][j] = (dp_left[i][j] + dp) % m;
                dp_diagonal[i][j] = (dp_diagonal[i][j] + dp) % m;

                if i == h - 1 && j == w - 1 {
                    println!("{}", dp);
                    return;
                }
            }
        }
    }
}
