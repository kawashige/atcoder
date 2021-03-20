use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        roads: [(usize, usize, usize); m]
    }

    let mut lists = vec![vec![]; n + 1];

    for (a, b, c) in roads {
        lists[a].push((b, c));
    }
    for i in 1..=n {
        let mut heap = BinaryHeap::new();
        let mut dist = vec![std::usize::MAX; n + 1];
        heap.push(Reverse((0, i)));

        while let Some(Reverse((t, n))) = heap.pop() {
            for &(b, c) in &lists[n] {
                if dist[b] > t + c {
                    dist[b] = t + c;
                    heap.push(Reverse((t + c, b)));
                }
            }
        }

        if dist[i] == std::usize::MAX {
            println!("-1");
        } else {
            println!("{}", dist[i]);
        }
    }
}
