use proconio::input;

fn main() {
    input! {
        s: usize
    }

    if s < 3 {
        println!("0");
        return;
    }

    let mut dp = vec![0; s + 1];
    dp[0] = 1;
    for i in 3..=s {
        for j in 0..=(i - 3) {
            dp[i] += dp[j];
            dp[i] %= 1_000_000_007;
        }
    }

    println!("{}", dp[s]);
}
