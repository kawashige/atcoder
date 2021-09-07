use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    const M: u64 = 998244353;

    let mut dp = vec![vec![vec![0; n + 1]; 2 * n]; 2 * n];
    let mut pair = vec![vec![false; 2 * n]; 2 * n];

    for (a, b) in ab {
        if a + 1 == b {
            dp[a - 1][b - 1][1] = 1;
        }
        pair[a - 1][b - 1] = true;
    }

    for l in (4..=(2 * n)).step_by(2) {
        for i in 0..=(2 * n - l) {
            if pair[i][i + l - 1] {
                for j in 1..=n {
                    if dp[i + 1][i + l - 2][j] > 0 {
                        dp[i][i + l - 1][j] += dp[i + 1][i + l - 2][j];
                    }
                }
            }

            if pair[i][i + 1] {
                for j in 1..=n {
                    if dp[i + 2][i + l - 1][j] > 0 {
                        dp[i][i + l - 1][j] += dp[i + 2][i + l - 1][j] + 1;
                    }
                }
            }
        }
    }

    let mut r = 0;
    let mut factorial = 1;
    for i in 1..=n {
        factorial = factorial * i as u64 % M;
        r += factorial * dp[0][2 * n - 1][i] % M;
        r %= M
    }

    println!("{}", r);
}
