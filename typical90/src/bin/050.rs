use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
    }

    let m = 1_000_000_007;
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 0..n {
        dp[i + 1] += dp[i];
        dp[i + 1] %= m;
        if i + l <= n {
            dp[i + l] += dp[i];
            dp[i + l] %= m;
        }
    }

    println!("{}", dp[n]);
}
