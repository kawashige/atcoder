use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        x: [usize; n]
    }

    let mut dp = vec![vec![vec![0_u64; 50 * n + 1]; n + 1]; n + 1];
    dp[0][0][0] = 1;

    for i in 0..n {
        for j in 0..=i {
            for k in 0..dp[0][0].len() {
                if dp[i][j][k] > 0 {
                    dp[i + 1][j][k] += dp[i][j][k];
                    dp[i + 1][j + 1][k + x[i]] += dp[i][j][k];
                }
            }
        }
    }

    let mut count = 0;
    for i in 1..=n {
        count += dp[n][i][i * a];
    }

    println!("{}", count);
}
