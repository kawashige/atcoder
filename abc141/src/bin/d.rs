use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n]
    }

    let mut heap = BinaryHeap::from(a);
    for _ in 0..m {
        if let Some(v) = heap.pop() {
            heap.push(v / 2);
        }
    }

    let mut sum = 0;
    while let Some(v) = heap.pop() {
        sum += v;
    }

    println!("{}", sum);
}
