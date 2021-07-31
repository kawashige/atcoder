use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uv: [(usize, usize); m]
    }

    const M: i32 = 998244353;

    let mut dp = vec![vec![0; n]; k + 1];
    dp[0][0] = 1;

    for i in 0..k {
        let sum = dp[i].iter().fold(0, |acc, x| (acc + x) % M);
        for j in 0..n {
            dp[i + 1][j] = sum - dp[i][j];
            if dp[i + 1][j] < 0 {
                dp[i + 1][j] += M;
            }
        }

        for j in 0..m {
            let (u, v) = uv[j];
            dp[i + 1][u - 1] -= dp[i][v - 1];
            dp[i + 1][v - 1] -= dp[i][u - 1];
            if dp[i + 1][u - 1] < 0 {
                dp[i + 1][u - 1] += M;
            }
            if dp[i + 1][v - 1] < 0 {
                dp[i + 1][v - 1] += M;
            }
        }
    }

    println!("{}", dp[k][0])
}
