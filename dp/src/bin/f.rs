use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut dp = vec![vec![(0, None); t.len() + 1]; s.len() + 1];

    for i in 0..s.len() {
        for j in 0..t.len() {
            dp[i + 1][j + 1] = if s[i] == t[j] {
                (dp[i][j].0 + 1, Some((i, j)))
            } else if dp[i][j + 1].0 < dp[i + 1][j].0 {
                dp[i + 1][j]
            } else {
                dp[i][j + 1]
            }
        }
    }

    let mut prev = dp[s.len()][t.len()].1;
    let mut chars = vec![];
    while let Some((i, j)) = prev {
        chars.push(s[i]);
        prev = dp[i][j].1;
    }

    println!("{}", chars.into_iter().rev().collect::<String>());
}
