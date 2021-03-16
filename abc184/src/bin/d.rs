use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    let mut dp = vec![vec![vec![0.0; 101]; 101]; 101];
    dp[a][b][c] = 1.0;
    let mut p = 0.0;

    for i in a..100 {
        for j in b..100 {
            for k in c..100 {
                let sum = (i + j + k) as f64;
                dp[i + 1][j][k] += dp[i][j][k] * i as f64 / sum;
                dp[i][j + 1][k] += dp[i][j][k] * j as f64 / sum;
                dp[i][j][k + 1] += dp[i][j][k] * k as f64 / sum;
                p += dp[i][j][k] * (i + j + k) as f64 / sum;
            }
        }
    }

    println!("{}", p);
}
