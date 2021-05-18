use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

const MAX: usize = 10_000_000_000;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m]
    }

    let mut list = vec![vec![]; n];
    for (a, b, c) in abc {
        list[a - 1].push((b - 1, c));
        list[b - 1].push((a - 1, c));
    }

    let mut dist_s = vec![MAX; n];
    let mut heap = BinaryHeap::new();
    let mut seen = vec![false; n];
    heap.push(Reverse((0, 0)));
    dist_s[0] = 0;
    while let Some(Reverse((c, n))) = heap.pop() {
        if dist_s[n] < c {
            continue;
        }

        seen[n] = true;

        for (next, cost) in &list[n] {
            if !seen[*next] && dist_s[n] + cost < dist_s[*next] {
                dist_s[*next] = dist_s[n] + cost;
                heap.push(Reverse((dist_s[*next], *next)));
            }
        }
    }

    let mut dist_e = vec![MAX; n];
    let mut heap = BinaryHeap::new();
    let mut seen = vec![false; n];
    heap.push(Reverse((0, n - 1)));
    dist_e[n - 1] = 0;
    while let Some(Reverse((c, n))) = heap.pop() {
        if dist_e[n] < c {
            continue;
        }

        seen[n] = true;

        for (next, cost) in &list[n] {
            if !seen[*next] && dist_e[n] + cost < dist_e[*next] {
                dist_e[*next] = dist_e[n] + cost;
                heap.push(Reverse((dist_e[*next], *next)));
            }
        }
    }

    for i in 0..n {
        println!("{}", dist_s[i] + dist_e[i]);
    }
}
