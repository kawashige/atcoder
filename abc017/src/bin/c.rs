use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lrs: [(usize, usize, usize); n]
    }

    lrs.sort_unstable();

    let sum = lrs.iter().map(|(_, _, s)| *s).sum::<usize>();
    let mut heap = BinaryHeap::new();

    let mut max = 0;
    let mut i = 0;
    let mut tmp_sum = 0;
    for j in 1..=m {
        while i < n && lrs[i].0 <= j {
            tmp_sum += lrs[i].2;
            heap.push((Reverse(lrs[i].1), lrs[i].2));
            i += 1;
        }
        while let Some((Reverse(x), s)) = heap.peek() {
            if x < &j {
                tmp_sum -= s;
                heap.pop();
            } else {
                break;
            }
        }
        max = max.max(sum - tmp_sum);
    }

    println!("{}", max);
}
