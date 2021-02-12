use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        nums: [usize; n]
    }

    let mut seen = HashSet::new();
    let mut current = 1;
    let mut count = 0;
    while current != 2 {
        if seen.contains(&current) {
            println!("-1");
            return;
        }
        count += 1;
        seen.insert(current);
        current = nums[current - 1];
    }
    println!("{}", count)
}
