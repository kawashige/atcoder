use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [usize; n]
    }

    let mut dp = vec![1; n];
    for i in 0..n {
        for j in 0..i {
            if c[j] < c[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }

    println!("{}", n - dp.into_iter().max().unwrap());
}
