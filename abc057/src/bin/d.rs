use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        mut v: [u64; n]
    }

    v.sort_unstable();

    let mut dp = vec![(0, 0_u64); n + 1];
    dp[0] = (0, 1);

    for i in 0..n {
        for j in (1..=n).rev() {
            if dp[j - 1].1 > 0 {
                if dp[j].0 < dp[j - 1].0 + v[i] {
                    dp[j] = (dp[j - 1].0 + v[i], dp[j - 1].1);
                } else if dp[j].0 == dp[j - 1].0 + v[i] {
                    dp[j].1 += dp[j - 1].1;
                }
            }
        }
    }

    let mut max_i = a;
    let mut max_count = dp[a].1;
    for i in (a + 1)..=b {
        let max_v = dp[max_i].0 * i as u64;
        let current_v = dp[i].0 * max_i as u64;
        if current_v > max_v {
            max_i = i;
            max_count = dp[i].1;
        } else if current_v == max_v {
            max_count += dp[i].1;
        }
    }

    println!("{}", dp[max_i].0 as f64 / max_i as f64);
    println!("{}", max_count);
}
