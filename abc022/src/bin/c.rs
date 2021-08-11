use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvl: [(usize, usize, usize); m]
    }

    let mut list = vec![vec![]; n];
    for (u, v, l) in uvl {
        list[u - 1].push((v - 1, l));
        list[v - 1].push((u - 1, l));
    }

    let mut min_d = std::usize::MAX;

    for (i, d) in &list[0] {
        let mut dist = vec![std::usize::MAX; n];
        dist[*i] = *d;
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(*d), *i));

        while let Some((Reverse(d), v)) = heap.pop() {
            if dist[v] < d {
                continue;
            }

            for (next, l) in &list[v] {
                if !(next == &0 && v == *i) && dist[*next] > dist[v] + l {
                    dist[*next] = dist[v] + l;
                    heap.push((Reverse(d + l), *next));
                }
            }
        }

        min_d = min_d.min(dist[0]);
    }

    if min_d == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", min_d);
    }
}
