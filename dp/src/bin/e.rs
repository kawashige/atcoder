use proconio::input;
fn main() {
    input! {
        n: usize,
        w: u64,
        wv: [(u64, usize); n]
    }

    let mut dp = vec![w + 1; 100001];
    dp[0] = 0;

    for i in 0..n {
        for j in (0..(100001 - wv[i].1)).rev() {
            dp[j + wv[i].1] = std::cmp::min(dp[j + wv[i].1], dp[j] + wv[i].0);
        }
    }

    println!("{}", (0..100001).rev().find(|i| dp[*i] <= w).unwrap_or(0));
}
