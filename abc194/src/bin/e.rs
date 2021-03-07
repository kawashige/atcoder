use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; n]
    }

    let mut map = BTreeMap::new();
    for (i, num) in a.into_iter().enumerate() {
        (*map.entry(num).or_insert(vec![0])).push(i + 1);
    }

    for i in 0..=1_500_000 {
        if let Some(nums) = map.get_mut(&i) {
            nums.push(n + 1);
            if (1..nums.len()).any(|i| nums[i] - nums[i - 1] > m) {
                println!("{}", i);
                return;
            }
        } else {
            println!("{}", i);
            return;
        }
    }
}
