use proconio::input;

fn main() {
    input! {
        n: usize,
        mut nums: [i32; n]
    }

    nums.sort_unstable_by(|a, b| b.cmp(&a));

    let mut sums = vec![0_i128; n];
    sums[n - 1] = nums[n - 1] as i128;

    for i in (0..(n - 1)).rev() {
        sums[i] = sums[i + 1] + nums[i] as i128;
    }

    let mut sum = 0_i128;
    for i in 0..(n - 1) {
        sum += nums[i] as i128 * (n as i128 - 1 - i as i128) - sums[i + 1];
    }

    println!("{}", sum);
}
