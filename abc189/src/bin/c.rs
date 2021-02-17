use proconio::input;

fn main() {
    input! {
        n: usize,
        nums: [usize; n]
    }

    let mut count = 0;
    for i in 0..nums.len() {
        let mut min = std::usize::MAX;
        for j in i..nums.len() {
            min = std::cmp::min(min, nums[j]);
            count = std::cmp::max(count, min * (j - i + 1));
        }
    }
    println!("{}", count);
}
