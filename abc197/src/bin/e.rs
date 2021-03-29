use proconio::input;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    input! {
        n: usize,
        xc: [(i128, i128); n]
    }

    let mut map = BTreeMap::new();
    for (x, c) in xc {
        (*map.entry(c).or_insert(BTreeSet::new())).insert(x);
    }

    let mut dp = vec![(0, 0); 2];
    for (_, s) in map.into_iter() {
        let min = s.iter().next().unwrap();
        let max = s.iter().next_back().unwrap();
        let dp0 = (
            std::cmp::min(
                dp[0].0 + (dp[0].1 - max).abs() + (max - min).abs(),
                dp[1].0 + (dp[1].1 - max).abs() + (max - min).abs(),
            ),
            *min,
        );
        let dp1 = (
            std::cmp::min(
                dp[0].0 + (dp[0].1 - min).abs() + (max - min).abs(),
                dp[1].0 + (dp[1].1 - min).abs() + (max - min).abs(),
            ),
            *max,
        );
        dp[0] = dp0;
        dp[1] = dp1;
    }

    if dp[0].0 + dp[0].1.abs() < dp[1].0 + dp[1].1.abs() {
        println!("{}", dp[0].0 + dp[0].1.abs());
    } else {
        println!("{}", dp[1].0 + dp[1].1.abs());
    }
}
