use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; 3 * n]
    }

    let mut max_heap = BinaryHeap::new();
    let mut sum = 0;
    for i in (2 * n)..(3 * n) {
        max_heap.push(a[i]);
        sum += a[i];
    }
    let mut min = vec![0; 3 * n];
    min[2 * n] = sum;

    for i in (n..(2 * n)).rev() {
        min[i] = min[i + 1];
        if let Some(x) = max_heap.peek() {
            let x = *x;
            if x > a[i] {
                max_heap.pop();
                max_heap.push(a[i]);
                min[i] += a[i];
                min[i] -= x;
            }
        }
    }

    let mut min_heap = BinaryHeap::new();
    let mut sum = 0;
    for i in 0..n {
        min_heap.push(Reverse(a[i]));
        sum += a[i];
    }
    let mut max = vec![0; 3 * n];
    max[n - 1] = sum;

    for i in n..(2 * n) {
        max[i] = max[i - 1];
        if let Some(Reverse(x)) = min_heap.peek() {
            let x = *x;
            if x < a[i] {
                min_heap.pop();
                min_heap.push(Reverse(a[i]));
                max[i] += a[i];
                max[i] -= x;
            }
        }
    }

    let mut r = std::i64::MIN;
    for i in n..=(2 * n) {
        r = std::cmp::max(r, max[i - 1] - min[i]);
    }

    println!("{}", r);
}
