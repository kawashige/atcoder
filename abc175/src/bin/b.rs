use proconio::input;

fn main() {
    input! {
        n: usize,
        mut nums: [usize; n]
    }

    nums.sort_unstable();

    let mut count = 0;
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] == nums[j] {
                continue;
            }
            for k in (j + 1)..nums.len() {
                if nums[k] == nums[j] {
                    continue;
                }
                if nums[k] < nums[i] + nums[j] {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
