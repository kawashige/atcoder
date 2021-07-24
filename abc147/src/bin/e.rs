use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h],
        b: [[i32; w]; h],
    }

    let mut dp = vec![vec![vec![false; 12801]; w]; h];

    for i in (0..h).rev() {
        for j in (0..w).rev() {
            let d = (a[i][j] - b[i][j]).abs() as usize;
            if i == h - 1 && j == w - 1 {
                dp[i][j][6400 - d] = true;
                dp[i][j][6400 + d] = true;
            }
            for k in 0..=12800 {
                if i < h - 1 && dp[i + 1][j][k] {
                    if k + d <= 12800 {
                        dp[i][j][k + d] = true;
                    }
                    if k >= d {
                        dp[i][j][k - d] = true;
                    }
                }
                if j < w - 1 && dp[i][j + 1][k] {
                    if k + d <= 12800 {
                        dp[i][j][k + d] = true;
                    }
                    if k >= d {
                        dp[i][j][k - d] = true;
                    }
                }
            }
        }
    }

    for i in 0..=6400 {
        if dp[0][0][6400 + i] || dp[0][0][6400 - i] {
            println!("{}", i);
            return;
        }
    }
}
