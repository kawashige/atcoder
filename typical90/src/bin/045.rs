use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xy: [(i64, i64); n]
    }

    let mut d = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            d[i][j] = (xy[i].0 - xy[j].0).pow(2) + (xy[i].1 - xy[j].1).pow(2);
        }
    }

    let mut cost = vec![0; 1 << n];

    for i in 1..(1 << n) {
        for j in 0..n {
            for k in 0..j {
                if (i >> j & 1) > 0 && ((i >> k) & 1) > 0 {
                    cost[i] = cost[i].max(d[j][k]);
                }
            }
        }
    }

    let mut dp = vec![vec![std::i64::MAX; 1 << n]; k + 1];
    dp[0][0] = 0;

    for i in 1..=k {
        for j in 1..(1 << n) {
            let mut k = j;
            while k != 0 {
                dp[i][j] = dp[i][j].min(dp[i - 1][j - k].max(cost[k]));
                k = (k - 1) & j;
            }
        }
    }

    println!("{}", dp[k][(1 << n) - 1]);
}
