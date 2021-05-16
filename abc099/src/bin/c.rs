use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let mut nums = Vec::new();
    if n >= 6 {
        nums.push(6);
        while nums.last().unwrap() * 6 <= n {
            nums.push(nums.last().unwrap() * 6);
        }
    }
    if n >= 9 {
        nums.push(9);
        while nums.last().unwrap() * 9 <= n {
            nums.push(nums.last().unwrap() * 9);
        }
    }
    nums.sort_unstable();

    let mut dp = vec![0; n + 1];

    for x in &nums {
        dp[*x] = 1;
    }

    let mut max_i = 0;
    for i in 0..=n {
        for x in &nums {
            if &i < x || dp[i - x] == 0 {
                continue;
            }
            dp[i] = if dp[i] == 0 {
                dp[i - x] + 1
            } else {
                std::cmp::min(dp[i - x] + 1, dp[i])
            };
        }
        if dp[i] > 0 {
            max_i = i;
        }
    }

    println!("{}", dp[max_i] + n - max_i);
}
