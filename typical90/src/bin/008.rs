use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars
    }

    let m = 1_000_000_007;
    let mut dp = vec![0; 7];

    for c in s {
        match c {
            'a' => {
                dp[0] += 1;
            }
            't' => {
                dp[1] += dp[0];
                dp[1] %= m;
            }
            'c' => {
                dp[2] += dp[1];
                dp[2] %= m;
            }
            'o' => {
                dp[3] += dp[2];
                dp[3] %= m;
            }
            'd' => {
                dp[4] += dp[3];
                dp[4] %= m;
            }
            'e' => {
                dp[5] += dp[4];
                dp[5] %= m;
            }
            'r' => {
                dp[6] += dp[5];
                dp[6] %= m;
            }
            _ => {}
        }
    }

    println!("{}", dp[6]);
}
