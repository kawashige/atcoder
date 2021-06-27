use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }

    let mut sum = vec![0; n + 1];
    sum[1] = a[0];
    for i in 1..n {
        sum[i + 1] = sum[i] + a[i];
    }

    let mut indices = vec![vec![-1; n + 1]; n + 1];
    let mut pre = vec![vec![-1; n + 1]; n + 1];
    for i in 0..=n {
        for j in 1..=n {
            indices[i][j] = pre[j][(sum[i] % j as u64) as usize];
            pre[j][(sum[i] % j as u64) as usize] = i as i32;
        }
    }

    let mut dp = vec![vec![0; n + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        for j in 1..=n {
            if indices[i][j] != -1 {
                dp[i][j] = (dp[indices[i][j] as usize][j] + dp[indices[i][j] as usize][j - 1])
                    % 1_000_000_007;
            }
        }
    }

    println!("{}", dp[n].iter().sum::<u64>() % 1_000_000_007);
}
