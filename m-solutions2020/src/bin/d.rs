use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }

    let mut dp = vec![0_u64; n];

    for i in 1..n {
        let mut max = 1000;
        for j in 0..i {
            if a[i] > a[j] {
                let m = (max / a[j]) * a[i] + max % a[j];
                dp[i] = std::cmp::max(dp[i], m);
            }
            max = std::cmp::max(max, dp[j]);
        }
    }

    println!("{}", dp.into_iter().max().unwrap().max(1000));
}
