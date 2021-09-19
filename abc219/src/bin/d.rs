use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        ab: [(usize, usize); n]
    }

    let mut dp = vec![vec![vec![std::usize::MAX; y + 1]; x + 1]; n];
    dp[0][ab[0].0.min(x)][ab[0].1.min(y)] = 1;

    for i in 1..n {
        for j in 1..=x {
            for k in 1..=y {
                if dp[i - 1][j][k] != std::usize::MAX {
                    dp[i][j][k] = dp[i][j][k].min(dp[i - 1][j][k]);
                    dp[i][(j + ab[i].0).min(x)][(k + ab[i].1).min(y)] =
                        dp[i][(j + ab[i].0).min(x)][(k + ab[i].1).min(y)].min(dp[i - 1][j][k] + 1);
                }
            }
        }
        dp[i][ab[i].0.min(x)][ab[i].1.min(y)] = 1;
    }

    if dp[n - 1][x][y] == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", dp[n - 1][x][y]);
    }
}
