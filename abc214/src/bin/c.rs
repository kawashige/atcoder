use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n]
    }

    let mut heap = BinaryHeap::new();

    for i in 0..n {
        heap.push((Reverse(t[i]), i));
    }

    let mut r = vec![0; n];
    let mut rest = n;

    while rest > 0 {
        if let Some((Reverse(t), i)) = heap.pop() {
            if r[i] > 0 {
                continue;
            }
            r[i] = t;
            rest -= 1;
            heap.push((Reverse(t + s[i]), (i + 1) % n));
        }
    }

    for x in r {
        println!("{}", x);
    }
}
