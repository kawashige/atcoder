use proconio::input;
use proconio::marker::Chars;

const M: usize = 998244353;

fn mod_pow(x: usize, n: usize) -> usize {
    if n == 0 {
        1
    } else if n % 2 == 0 {
        let r = mod_pow(x, n / 2);
        r * r % M
    } else {
        x * mod_pow(x, n - 1) % M
    }
}

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    // let mut count = vec![0; 10];
    // for c in &s {
    //     count[*c as usize - b'A' as usize] += 1;
    // }

    let mut r = 0;

    // for c in &count {
    //     r += mod_pow(2, *c) - 1;
    //     r %= M;
    // }

    // println!("r1: {}", r);

    let mut dp = vec![vec![vec![0; 1 << 10]; 10]; n + 1];

    for i in 0..n {
        for j in 0..10 {
            for k in 1..dp[0].len() {
                if j == (s[i] as usize - b'A' as usize) {
                    if dp[i][j][k] > 0 {
                        dp[i + 1][j][k] += (dp[i][j][k] * 2 + 1) % M;
                        dp[i + 1][j][k] %= M;
                    }
                } else {
                    if dp[i][j][k] > 0 {
                        dp[i + 1][j][k] += dp[i][j][k];
                        dp[i + 1][j][k] %= M;
                        let x = 1 << (s[i] as usize - b'A' as usize);
                        if k & x == 0 {
                            dp[i + 1][s[i] as usize - b'A' as usize][k | x] += dp[i][j][k];
                            dp[i + 1][s[i] as usize - b'A' as usize][k | x] %= M;
                        }
                    }
                }
            }
        }
        dp[i + 1][s[i] as usize - b'A' as usize][1 << (s[i] as usize - b'A' as usize)] += 1;
        dp[i + 1][s[i] as usize - b'A' as usize][1 << (s[i] as usize - b'A' as usize)] %= M;
    }

    // println!("dp: {:?}", dp);

    for i in 0..dp[0][0].len() {
        for j in 0..10 {
            r += dp[n][j][i];
            r %= M;
        }
    }

    println!("{}", r);
}
