use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut heap = BinaryHeap::new();
    let mut b = 0;
    for _ in 0..q {
        input! {
            query: usize
        }

        if query == 1 {
            input! {
                x: i64
            }
            heap.push(Reverse(x - b));
        } else if query == 2 {
            input! {
                x: i64
            }
            b += x;
        } else {
            if let Some(Reverse(x)) = heap.pop() {
                println!("{}", x + b);
            }
        }
    }
}
