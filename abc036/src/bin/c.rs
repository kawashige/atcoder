use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut nums = a.into_iter().enumerate().collect::<Vec<(_, _)>>();
    nums.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    let mut x = 0;
    let mut nums = (0..n)
        .map(|i| {
            if i > 0 && nums[i - 1].1 < nums[i].1 {
                x += 1;
            }
            (nums[i].0, x)
        })
        .collect::<Vec<(_, _)>>();
    nums.sort_unstable();

    for (_, x) in nums {
        println!("{}", x);
    }
}
