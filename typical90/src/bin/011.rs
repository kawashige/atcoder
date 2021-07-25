use std::usize;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut dcs: [(usize, usize, u64); n]
    }

    dcs.sort_unstable();

    let mut dp = vec![0; 5001];

    for i in 0..n {
        if dcs[i].0 < dcs[i].1 {
            continue;
        }
        for j in (1..(dcs[i].0 - dcs[i].1 + 1).min(dp.len())).rev() {
            if dp[j] > 0 {
                dp[j + dcs[i].1] = dp[j + dcs[i].1].max(dp[j] + dcs[i].2);
            }
        }
        dp[dcs[i].1] = dp[dcs[i].1].max(dcs[i].2);
    }

    println!("{}", dp.iter().max().unwrap());
}
