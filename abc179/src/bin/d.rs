use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ranges: [[usize; 2]; k]
    }

    let mut nums = ranges
        .into_iter()
        .map(|v| (v[0]..=v[1]).collect::<Vec<usize>>())
        .flatten()
        .collect::<Vec<usize>>();
    nums.sort_unstable();
    nums.dedup();

    let mut dp = vec![0; n + 1];
    dp[1] = 1;

    for i in 2..=n {
        for j in 0..nums.len() {
            if nums[j] >= i {
                break;
            }
            dp[i] += dp[i - nums[j]] as u128 % 998244353;
        }
    }

    println!("{}", dp[n] % 998244353);
}
