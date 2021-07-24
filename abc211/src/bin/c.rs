use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }
    const M: usize = 1_000_000_007;

    let mut dp = vec![0; 8];

    for i in 0..s.len() {
        match s[i] {
            'c' => dp[0] = (dp[0] + 1) % M,
            'h' => dp[1] = (dp[1] + dp[0]) % M,
            'o' => dp[2] = (dp[2] + dp[1]) % M,
            'k' => dp[3] = (dp[3] + dp[2]) % M,
            'u' => dp[4] = (dp[4] + dp[3]) % M,
            'd' => dp[5] = (dp[5] + dp[4]) % M,
            'a' => dp[6] = (dp[6] + dp[5]) % M,
            'i' => dp[7] = (dp[7] + dp[6]) % M,
            _ => {}
        }
    }

    println!("{}", dp[7]);
}
