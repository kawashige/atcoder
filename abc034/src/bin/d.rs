use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut wp: [(f64, usize); n]
    }

    wp.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let mut dp = vec![(0.0, 0.0); k + 1];

    for i in 0..n {
        let (w, p) = &wp[i];
        for j in (0..k).rev() {
            if dp[j].0 > 0.0 {
                let new_p = (dp[j].0 * dp[j].1 + w * (*p as f64)) / (dp[j].1 + w);
                if dp[j + 1].0 < new_p {
                    dp[j + 1] = (new_p, w + dp[j].1);
                }
            }
        }
        if dp[1].0 < *p as f64 {
            dp[1] = (*p as f64, *w);
        }
    }

    println!("{}", dp[k].0);
}
