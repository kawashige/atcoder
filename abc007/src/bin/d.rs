use proconio::input;

fn count(x: u64) -> u64 {
    if x == 0 {
        return 1;
    }
    let chars = x
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();
    let n = chars.len();

    // dp[i桁目][smaller]
    let mut dp = vec![vec![0; 2]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        dp[i + 1][0] = if chars[i] != 4 && chars[i] != 9 {
            dp[i][0]
        } else {
            0
        };
        dp[i + 1][1] =
            dp[i][1] * 8 + dp[i][0] * (0..chars[i]).filter(|i| i != &4 && i != &9).count() as u64;
    }

    dp[n][0] + dp[n][1]
}

fn main() {
    input! {
        a: u64,
        b: u64
    }

    println!("{}", (b - a + 1) + count(a - 1) - count(b));
}
