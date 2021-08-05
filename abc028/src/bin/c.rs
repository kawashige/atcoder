use proconio::input;

fn main() {
    input! {
        v: [usize; 5]
    }

    let mut nums = Vec::new();
    for i in 0..5 {
        for j in (i + 1)..5 {
            for k in (j + 1)..5 {
                nums.push(v[i] + v[j] + v[k]);
            }
        }
    }

    nums.sort_unstable_by(|a, b| b.cmp(&a));
    println!("{}", nums[2]);
}
