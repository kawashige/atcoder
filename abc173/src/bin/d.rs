use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        mut a: [u128; n]
    }

    a.sort_unstable_by(|a, b| b.cmp(&a));

    let mut queue = VecDeque::new();
    queue.push_back(0);
    let mut sum = 0;
    for i in 0..a.len() {
        sum += queue.pop_front().unwrap();
        queue.push_back(a[i]);
        if i > 0 {
            queue.push_back(a[i]);
        }
    }
    println!("{}", sum);
}
