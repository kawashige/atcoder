use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut ab: [(u64, u64); n]
    }

    ab.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    let mut result: u64 = 0;
    let mut v = BinaryHeap::new();
    for _ in 0..k {
        result += if !v.is_empty() && !ab.is_empty() {
            if v.peek().unwrap() < &ab.last().unwrap().1 {
                let (a, b) = ab.pop().unwrap();
                v.push(a - b);
                b
            } else {
                v.pop().unwrap()
            }
        } else if v.is_empty() {
            let (a, b) = ab.pop().unwrap();
            v.push(a - b);
            b
        } else {
            v.pop().unwrap()
        }
    }

    println!("{}", result);
}
