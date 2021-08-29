use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    const M: usize = 998244353;

    let mut ab = a.into_iter().zip(b.into_iter()).collect::<Vec<_>>();
    ab.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut r = 0;

    let mut dp = vec![0; 5001];

    for i in 0..n {
        for j in 1..=ab[i].0 {
            if dp[j] > 0 {
                r += dp[j];
                r %= M;
            }
        }
        if ab[i].1 <= ab[i].0 {
            r += 1;
            r %= M;
        }

        let mut new_dp = vec![0; 5001];
        for j in 0..dp.len() {
            if dp[j] > 0 {
                new_dp[j] += dp[j];
                new_dp[j] %= M;

                if j + ab[i].1 <= 5000 {
                    new_dp[j + ab[i].1] += dp[j];
                    new_dp[j + ab[i].1] %= M;
                }
            }
        }
        new_dp[ab[i].1] += 1;
        new_dp[ab[i].1] %= M;
        dp = new_dp;
    }

    println!("{}", r);
}
