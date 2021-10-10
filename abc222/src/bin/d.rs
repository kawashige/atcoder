use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    const M: usize = 998244353;

    let mut dp = vec![vec![0; 3001]; n + 1];
    for i in 0..dp[0].len() {
        dp[0][i] = 1;
    }

    for i in 0..n {
        for j in 0..dp[i].len() {
            if j > 0 {
                dp[i + 1][j] = dp[i + 1][j - 1];
            }
            if a[i] <= j && j <= b[i] {
                dp[i + 1][j] += dp[i][j];
                dp[i + 1][j] %= M;
            }
        }
    }

    println!("{}", dp[n][3000]);
}
