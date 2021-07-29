use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        t: u64,
        a: [u64; n],
        abc: [(usize, usize, u64); m]
    }

    let mut list = vec![vec![]; n];
    let mut list2 = vec![vec![]; n];
    for (a, b, c) in abc {
        list[a - 1].push((b - 1, c));
        list2[b - 1].push((a - 1, c));
    }

    let mut dist = vec![std::u64::MAX; n];
    let mut heap = BinaryHeap::new();
    dist[0] = 0;
    heap.push((0, 0));

    while let Some((v, d)) = heap.pop() {
        if dist[v] < d {
            continue;
        }

        for (next, c) in &list[v] {
            if dist[*next] > dist[v] + c {
                dist[*next] = dist[v] + c;
                heap.push((*next, dist[*next]));
            }
        }
    }

    let mut dist2 = vec![std::u64::MAX; n];
    let mut heap = BinaryHeap::new();
    dist2[0] = 0;
    heap.push((0, 0));

    while let Some((v, d)) = heap.pop() {
        if dist2[v] < d {
            continue;
        }

        for (next, c) in &list2[v] {
            if dist2[*next] > dist2[v] + c {
                dist2[*next] = dist2[v] + c;
                heap.push((*next, dist2[*next]));
            }
        }
    }

    let mut max = 0;

    for i in 0..n {
        if dist[i] == std::u64::MAX || dist2[i] == std::u64::MAX || dist[i] + dist2[i] > t {
            continue;
        }
        max = max.max(a[i] * (t - dist[i] - dist2[i]));
    }

    println!("{}", max);
}
