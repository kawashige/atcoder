use proconio::input;

fn main() {
    input! {
        n: usize
    }
    const M: usize = 10_007;

    let mut dp = vec![0; n + 1];
    if n + 1 > 3 {
        dp[3] = 1;
    }
    for i in 4..=n {
        dp[i] = (dp[i - 3] + dp[i - 2] + dp[i - 1]) % M;
    }

    println!("{}", dp[n]);
}
