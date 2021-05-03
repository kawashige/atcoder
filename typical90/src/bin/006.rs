use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    }

    let mut heap = BinaryHeap::new();
    let mut r = String::new();
    let mut last = 0;

    for i in 0..n {
        heap.push(Reverse((s[i], i)));
        if k - r.len() == n - i {
            while let Some(Reverse((c, j))) = heap.pop() {
                if j < last {
                    continue;
                }
                last = j;
                r.push(c);
                break;
            }
        }
    }

    println!("{}", r);
}
