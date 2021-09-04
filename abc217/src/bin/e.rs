use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use proconio::input;

fn main() {
    input! {
        q: usize
    }

    let mut heap = BinaryHeap::new();
    let mut deque = VecDeque::new();

    for _ in 0..q {
        input! {
            c: usize
        }

        if c == 1 {
            input! {
                x: usize
            }

            deque.push_back(x);
        } else if c == 2 {
            if heap.is_empty() {
                if let Some(x) = deque.pop_front() {
                    println!("{}", x);
                }
            } else {
                if let Some(Reverse(x)) = heap.pop() {
                    println!("{}", x);
                }
            }
        } else {
            let q = deque.clone();
            deque.clear();
            for x in q {
                heap.push(Reverse(x));
            }
        }
    }
}
