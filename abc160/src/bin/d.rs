use std::collections::{HashMap, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize
    }

    let mut count = HashMap::new();
    for i in 0..n {
        let mut seen = vec![false; n];
        let mut queue = VecDeque::new();
        queue.push_back((i, 0));
        while let Some((j, d)) = queue.pop_front() {
            if seen[j] {
                continue;
            }

            seen[j] = true;
            *count.entry(d).or_insert(0) += 1;

            if 0 < j {
                queue.push_back((j - 1, d + 1));
            }
            if j < n - 1 {
                queue.push_back((j + 1, d + 1));
            }
            if j == x - 1 {
                queue.push_back((y - 1, d + 1));
            }
            if j == y - 1 {
                queue.push_back((x - 1, d + 1));
            }
        }
    }

    for i in 1..n {
        println!("{}", count.get(&i).unwrap_or(&0) / 2);
    }
}
