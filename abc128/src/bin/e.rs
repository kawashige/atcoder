use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        stx: [(i32, i32, i32); n],
        d: [i32; q]
    }

    let mut stx = stx
        .into_iter()
        .map(|(s, t, x)| (s - x, t - x, x))
        .collect::<Vec<_>>();
    stx.sort_unstable();

    let mut d = d.into_iter().zip(0..).collect::<Vec<_>>();
    d.sort_unstable();

    let mut result = vec![-1; q];
    let mut heap = BinaryHeap::new();
    let mut j = 0;

    for (t, i) in d {
        while j < n && stx[j].0 <= t {
            heap.push((Reverse(stx[j].2), Reverse(stx[j].1)));
            j += 1;
        }

        while let Some((Reverse(x), Reverse(e))) = heap.peek() {
            if e > &t {
                result[i] = *x as i32;
                break;
            }
            heap.pop();
        }
    }

    for x in result {
        println!("{}", x);
    }
}
