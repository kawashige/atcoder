use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut dp = vec![vec![0; 2]; s.len() + 1];

    for i in 0..s.len() {
        if s[i] == '0' {
            dp[i + 1][0] = dp[i][1];
            dp[i + 1][1] = dp[i][0] + 1;
        } else {
            dp[i + 1][0] = dp[i][1] + 1;
            dp[i + 1][1] = dp[i][0];
        }
    }

    println!("{}", std::cmp::min(dp[s.len()][0], dp[s.len()][1]));
}
