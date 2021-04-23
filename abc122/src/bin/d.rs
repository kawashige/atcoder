use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let m = 1_000_000_007;

    let mut dp = vec![vec![vec![vec![0; 4]; 4]; 4]; 2];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                dp[0][i][j][k] = 1;
            }
        }
    }
    dp[0][0][2][1] = 0;
    dp[0][2][0][1] = 0;
    dp[0][0][1][2] = 0;

    let mut l = 0;
    for _ in 4..=n {
        l = (l + 1) % 2;

        for i in 0..4 {
            for j in 0..4 {
                for o in 0..4 {
                    for k in 0..4 {
                        if (o == 0 && j == 2 && k == 1)
                            || (o == 0 && i == 2 && k == 1)
                            || (i == 0 && j == 2 && k == 1)
                            || (i == 0 && j == 1 && k == 2)
                            || (i == 2 && j == 0 && k == 1)
                        {
                            continue;
                        }
                        dp[l][i][j][k] += dp[(l + 1) % 2][o][i][j];
                        dp[l][i][j][k] %= m;
                    }
                    dp[(l + 1) % 2][o][i][j] = 0;
                }
            }
        }
    }

    let mut sum = 0;
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                sum += dp[l][i][j][k];
                sum %= m;
            }
        }
    }

    println!("{}", sum);
}
