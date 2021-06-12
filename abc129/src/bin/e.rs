use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        l: Chars
    }
    let m = 1_000_000_007;

    let mut dp = vec![vec![vec![0; 2]; 2]; l.len() + 1];
    dp[0][0][0] = 1;
    for i in 0..l.len() {
        if l[i] == '0' {
            // smaller / not smaller -> 0
            dp[i + 1][1][0] = (dp[i][1][0] + dp[i][1][1]) % m;
            dp[i + 1][0][0] = (dp[i][0][0] + dp[i][0][1]) % m;

            // smaller -> 1
            dp[i + 1][1][1] = ((dp[i][1][0] + dp[i][1][1]) % m) * 2 % m;
        } else {
            // smaller / not smaller -> 0
            dp[i + 1][1][0] =
                (((dp[i][1][0] + dp[i][1][1]) % m + dp[i][0][0]) % m + dp[i][0][1]) % m;

            // smaller / not smaller -> 1
            dp[i + 1][1][1] = ((dp[i][1][0] + dp[i][1][1]) % m) * 2 % m;
            dp[i + 1][0][1] = ((dp[i][0][0] + dp[i][0][1]) % m) * 2 % m;
        }
    }

    let mut r = dp[l.len()][0][0] + dp[l.len()][0][1];
    r %= m;
    r += dp[l.len()][1][0];
    r %= m;
    r += dp[l.len()][1][1];
    r %= m;

    println!("{}", r);
}
