use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
        mut p: [u128; a],
        mut q: [u128; b],
        mut r: [u128; c]
    }

    p.sort_unstable_by(|a, b| b.cmp(&a));
    q.sort_unstable_by(|a, b| b.cmp(&a));
    r.sort_unstable_by(|a, b| b.cmp(&a));

    let mut heap = BinaryHeap::new();
    for n in p.into_iter().take(x).chain(q.into_iter().take(y)) {
        heap.push(Reverse(n));
    }

    for n in r {
        if &Reverse(n) < heap.peek().unwrap() {
            heap.pop();
            heap.push(Reverse(n));
        } else {
            break;
        }
    }

    let mut sum = 0;
    while let Some(Reverse(n)) = heap.pop() {
        sum += n;
    }

    println!("{}", sum);



}
