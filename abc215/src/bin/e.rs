use proconio::input;
use proconio::marker::Chars;

const M: usize = 998244353;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut r = 0;
    let mut dp = vec![vec![vec![0; 10]; 1 << 10]; n + 1];

    for i in 0..n {
        let idx = s[i] as usize - b'A' as usize;
        for j in 1..dp[0].len() {
            for k in 0..10 {
                if dp[i][j][k] > 0 {
                    dp[i + 1][j][k] += dp[i][j][k];
                    dp[i + 1][j][k] %= M;

                    if idx == k || j & 1 << idx == 0 {
                        dp[i + 1][j | 1 << idx][idx] += dp[i][j][k];
                        dp[i + 1][j | 1 << idx][idx] %= M;
                    }
                }
            }
        }
        dp[i + 1][1 << idx][idx] += 1;
        dp[i + 1][1 << idx][idx] %= M;
    }

    for i in 0..(1 << 10) {
        for j in 0..10 {
            r += dp[n][i][j];
            r %= M;
        }
    }

    println!("{}", r);
}
