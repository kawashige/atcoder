use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut dp = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 0..w {
            dp[i][j] = if i > 0 && j > 0 {
                (dp[i - 1][j]
                    + if s[i - 1][j] == '.' && s[i][j] == '#' {
                        1
                    } else {
                        0
                    })
                .min(
                    dp[i][j - 1]
                        + if s[i][j - 1] == '.' && s[i][j] == '#' {
                            1
                        } else {
                            0
                        },
                )
            } else if i > 0 {
                dp[i - 1][j]
                    + if s[i - 1][j] == '.' && s[i][j] == '#' {
                        1
                    } else {
                        0
                    }
            } else if j > 0 {
                dp[i][j - 1]
                    + if s[i][j - 1] == '.' && s[i][j] == '#' {
                        1
                    } else {
                        0
                    }
            } else {
                if s[i][j] == '.' {
                    0
                } else {
                    1
                }
            };
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
