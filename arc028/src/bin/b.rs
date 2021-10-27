use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [usize; n]
    }

    let mut heap = BinaryHeap::new();

    for i in 0..n {
        heap.push((&x[i], i + 1));
        if heap.len() > k {
            heap.pop();
        }
        if heap.len() == k {
            println!("{}", heap.peek().unwrap().1);
        }
    }
}
