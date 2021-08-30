use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize
    }

    let skip = vec![0, b, b + g, b + g + r];
    let mut dp = vec![vec![std::i32::MAX; r + g + b + 1]; 2001];
    dp[0][r + g + b] = 0;

    for i in 0..(dp.len() - 1) {
        for j in 0..dp[0].len() {
            if dp[i][j] != std::i32::MAX {
                if skip.contains(&j) {
                    dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
                }
                if j > 0 {
                    let origin = if j <= b {
                        100
                    } else if j <= b + g {
                        0
                    } else {
                        -100
                    };
                    let m = (i as i32 - 1000 - origin).abs();
                    dp[i + 1][j - 1] = dp[i + 1][j - 1].min(dp[i][j] + m);
                }
            }
        }
    }

    println!("{}", dp[2000][0]);
}
