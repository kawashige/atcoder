use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i32; n]
    }

    let mut dp = vec![0; n];
    for i in 1..n {
        dp[i] = dp[i - 1] + (h[i] - h[i - 1]).abs();
        if i > 1 {
            dp[i] = std::cmp::min(dp[i], dp[i - 2] + (h[i] - h[i - 2]).abs())
        }
    }

    println!("{}", dp[n - 1]);
}
