use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    const M: u64 = 998244353;

    let mut binom = vec![vec![0; 2 * n]; 2 * n];
    binom[0][0] = 1;
    for i in 1..(2 * n) {
        binom[i][0] = 1;
        binom[i][i] = 1;
        for j in 0..(i - 1) {
            binom[i][j + 1] = (binom[i - 1][j] + binom[i - 1][j + 1]) % M;
        }
    }

    let mut can = vec![vec![false; 2 * n]; 2 * n];
    for (a, b) in ab {
        can[a - 1][b - 1] = true;
        can[b - 1][a - 1] = true;
    }

    let mut dp = vec![vec![0; 2 * n + 1]; 2 * n + 1];

    for i in 0..(2 * n + 1) {
        dp[i][0] = 1;
    }

    for j in 1..=n {
        for i in 0..(2 * (n - j) + 1) {
            dp[i][j] = 0;
            for k in 0..j {
                if can[i][i + (2 * k) + 1] {
                    let x = (dp[i + 1][k] * dp[i + (2 * k) + 2][j - k - 1]) % M;
                    dp[i][j] = (x * binom[j][k + 1] + dp[i][j]) % M;
                }
            }
        }
    }

    println!("{}", dp[0][n]);
}
