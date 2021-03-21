use proconio::input;

fn main() {
    input! {
        h: usize,
        n: usize,
        ab: [(usize, usize); n]
    }

    let mut dp = vec![std::usize::MAX; h + 1];
    dp[0] = 0;

    for i in 0..h {
        if dp[i] == std::usize::MAX {
            continue;
        }
        for j in 0..n {
            let k = std::cmp::min(h, i + ab[j].0);
            dp[k] = std::cmp::min(dp[k], dp[i] + ab[j].1);
        }
    }

    println!("{}", dp[h]);
}
