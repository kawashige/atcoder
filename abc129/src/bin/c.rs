use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m]
    }

    let m = 1_000_000_007;
    let mut broken = vec![false; n + 1];
    for i in a {
        broken[i] = true;
    }

    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        if broken[i] {
            continue;
        }

        dp[i] = if i == 1 {
            dp[i - 1]
        } else {
            (dp[i - 1] + dp[i - 2]) % m
        };
        dp[i] %= m;
    }

    println!("{}", dp[n]);
}
