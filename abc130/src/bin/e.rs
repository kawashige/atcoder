use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [i64; n],
        t: [i64; m]
    }

    const M: i64 = 1_000_000_007;

    let mut dp = vec![vec![0; m + 1]; n + 1];
    dp[0][0] = 1;

    for i in 0..=n {
        for j in 0..=m {
            if i > 0 && j > 0 && s[i - 1] == t[j - 1] {
                dp[i][j] += dp[i - 1][j - 1];
                dp[i][j] %= M;
            }
            if i > 0 {
                dp[i][j] += dp[i - 1][j];
                dp[i][j] %= M;
            }
            if j > 0 {
                dp[i][j] += dp[i][j - 1];
                dp[i][j] %= M;
            }
            if i > 0 && j > 0 {
                dp[i][j] -= dp[i - 1][j - 1];
                dp[i][j] %= M;
                if dp[i][j] < 0 {
                    dp[i][j] += M;
                }
            }
        }
    }

    println!("{}", dp[n][m]);
}
