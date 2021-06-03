use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        mut ab: [(usize, usize); n]
    }

    ab.sort_unstable();

    let mut dp = vec![0; 6000];

    for i in 0..n {
        for j in (0..t).rev() {
            if j == 0 {
                dp[ab[i].0] = std::cmp::max(dp[ab[i].0], ab[i].1);
            } else if dp[j] > 0 {
                dp[ab[i].0 + j] = std::cmp::max(dp[ab[i].0 + j], dp[j] + ab[i].1);
            }
        }
    }

    let r = dp.into_iter().max().unwrap();

    println!("{}", r);
}
