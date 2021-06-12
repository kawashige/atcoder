use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        abc: [(usize, usize, u64); n - 1],
        q: usize,
        k: usize,
        xy: [(usize, usize); q]
    }

    let mut list = vec![vec![]; n];
    for (a, b, c) in abc {
        list[a - 1].push((b - 1, c));
        list[b - 1].push((a - 1, c));
    }

    let mut dist = vec![std::u64::MAX; n];
    dist[k - 1] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, k - 1)));

    while let Some(Reverse((t, node))) = heap.pop() {
        if dist[node] < t {
            continue;
        }

        for (next, next_t) in &list[node] {
            if dist[*next] > dist[node] + next_t {
                dist[*next] = dist[node] + next_t;
                heap.push(Reverse((dist[*next], *next)));
            }
        }
    }

    for (x, y) in xy {
        println!("{}", dist[x - 1] + dist[y - 1]);
    }
}
