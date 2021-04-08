use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n]
    }

    let mut dp = vec![0; w + 1];

    let mut max = 0;
    for i in 0..n {
        for j in (0..=(w - wv[i].0)).rev() {
            dp[j + wv[i].0] = std::cmp::max(dp[j + wv[i].0], dp[j] + wv[i].1);
            max = std::cmp::max(max, dp[j + wv[i].0]);
        }
    }

    println!("{}", max)
}
