use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let d = s
        .into_iter()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let mut dp = vec![vec![vec![false; 1000]; 4]; d.len() + 1];
    dp[0][0][0] = true;

    for i in 0..n {
        for j in 0..4 {
            for k in 0..1000 {
                if dp[i][j][k] {
                    dp[i + 1][j][k] = true;
                    if j < 3 {
                        dp[i + 1][j + 1][k * 10 + d[i]] = true;
                    }
                }
            }
        }
    }

    println!("{}", dp[n][3].iter().filter(|x| **x).count());
}
