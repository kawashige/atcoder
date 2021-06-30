use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        a: [usize; n]
    }

    let mut min = std::u64::MAX;

    for k in 1..=n {
        let mut dp = vec![vec![vec![0; k]; k]; n];
        dp[0][0][a[0] % k] = a[0] as u64;

        for i in 1..n {
            let x = a[i] % k;
            dp[i][0][x] = a[i] as u64;

            for j in 0..k {
                for l in 0..k {
                    if dp[i - 1][j][l] > 0 {
                        dp[i][j][l] = std::cmp::max(dp[i][j][l], dp[i - 1][j][l]);
                        if j < k - 1 {
                            dp[i][j + 1][(l + x) % k] = std::cmp::max(
                                dp[i][j + 1][(l + x) % k],
                                dp[i - 1][j][l] + a[i] as u64,
                            );
                        }
                    }
                }
            }
        }

        if dp[n - 1][k - 1][(x % k as u64) as usize] > 0 {
            min = std::cmp::min(
                min,
                (x - dp[n - 1][k - 1][(x % k as u64) as usize]) / (k as u64),
            );
        }
    }

    println!("{}", min);
}
