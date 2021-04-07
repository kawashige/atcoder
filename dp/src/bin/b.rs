use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i32; n]
    }

    let mut dp = vec![0_u64; n];

    for i in 1..n {
        let mut tmp = std::u64::MAX;
        for j in if i < k { 0 } else { i - k }..i {
            tmp = std::cmp::min(tmp, dp[j] + (h[i] - h[j]).abs() as u64);
        }
        dp[i] = tmp;
    }

    println!("{}", dp[n - 1]);
}
