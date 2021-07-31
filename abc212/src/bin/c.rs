use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i32; n],
        mut b: [i32; m],
    }

    let mut nums = a
        .into_iter()
        .map(|x| (x, 0))
        .chain(b.into_iter().map(|x| (x, 1)))
        .collect::<Vec<(i32, i32)>>();
    nums.sort_unstable();

    let mut min = std::i32::MAX;
    for i in 0..(nums.len() - 1) {
        if nums[i].1 != nums[i + 1].1 {
            min = min.min(nums[i + 1].0 - nums[i].0);
        }
    }

    println!("{}", min);
}
