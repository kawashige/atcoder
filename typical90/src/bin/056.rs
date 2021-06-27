use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; s + 1]; n];
    let mut pre = vec![vec![0; s + 1]; n];
    if ab[0].0 <= s {
        dp[0][ab[0].0] = 2;
    }
    if ab[0].1 <= s {
        dp[0][ab[0].1] = 3;
    }

    for i in 1..n {
        let (a, b) = ab[i];
        for j in 1..s {
            if dp[i - 1][j] != 0 {
                if j + a <= s {
                    dp[i][j + a] = 2;
                    pre[i][j + a] = j;
                }
                if j + b <= s {
                    dp[i][j + b] = 3;
                    pre[i][j + b] = j;
                }
            }
        }
    }

    if dp[n - 1][s] == 0 {
        println!("Impossible");
    } else {
        let mut r = Vec::with_capacity(n);
        let mut j = s;
        for i in (0..n).rev() {
            r.push(dp[i][j]);
            j = pre[i][j];
        }
        println!(
            "{}",
            r.into_iter()
                .rev()
                .map(|i| match i {
                    2 => 'A',
                    _ => 'B',
                })
                .collect::<String>()
        );
    }
}
