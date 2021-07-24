use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }

    let mut dp = vec![0; n];
    for i in 1..n {
        dp[i] = dp[i - 1] + (a[i - 1] - a[i]).abs();
        if i > 1 {
            dp[i] = dp[i].min(dp[i - 2] + (a[i - 2] - a[i]).abs());
        }
    }

    println!("{}", dp[n - 1]);
}
