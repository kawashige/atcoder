use proconio::input;

fn main() {
    input! {
        w: usize,
        n: usize,
        m: usize,
        ab: [(usize, usize); n]
    }

    let mut dp = vec![vec![vec![0; w + 1]; m + 1]; n + 1];

    for i in 0..n {
        for j in 0..=m {
            for k in 0..=w {
                if dp[i][j][k] > 0 {
                    dp[i + 1][j][k] = dp[i + 1][j][k].max(dp[i][j][k]);
                    if j + 1 <= m && k + ab[i].0 <= w {
                        dp[i + 1][j + 1][k + ab[i].0] =
                            dp[i + 1][j + 1][k + ab[i].0].max(dp[i][j][k] + ab[i].1);
                    }
                }
            }
        }
        if ab[i].0 <= w {
            dp[i + 1][1][ab[i].0] = dp[i + 1][1][ab[i].0].max(ab[i].1);
        }
    }

    println!("{}", dp[n].iter().flatten().max().unwrap())
}
