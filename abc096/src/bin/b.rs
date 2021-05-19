use proconio::input;

fn main() {
    input! {
        mut nums: [usize; 3],
        k: usize
    }

    nums.sort_unstable();

    println!("{}", nums[0] + nums[1] + nums[2] * 2_usize.pow(k as u32));
}
