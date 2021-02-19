use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        peoples: [(usize, usize); n]
    }

    let mut queue = BinaryHeap::new();
    let mut sum_a: u128 = 0;
    for (a, b) in peoples {
        sum_a += a as u128;
        queue.push(a * 2 + b);
    }
    let mut count = 0;
    let mut sum_b: u128 = 0;
    while sum_b <= sum_a {
        sum_b += queue.pop().unwrap() as u128;
        count += 1;
    }
    println!("{}", count);
}
