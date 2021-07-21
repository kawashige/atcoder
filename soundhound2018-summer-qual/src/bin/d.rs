use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        uvab: [(usize, usize, usize, usize); m]
    }

    let mut list_yen = vec![vec![]; n];
    let mut list_snuke = vec![vec![]; n];

    for (u, v, a, b) in uvab {
        list_yen[u - 1].push((v - 1, a));
        list_yen[v - 1].push((u - 1, a));
        list_snuke[u - 1].push((v - 1, b));
        list_snuke[v - 1].push((u - 1, b));
    }

    let mut d_yen = vec![std::usize::MAX; n];
    let mut heap = BinaryHeap::new();
    heap.push((s - 1, 0));
    d_yen[s - 1] = 0;

    while let Some((v, d)) = heap.pop() {
        if d_yen[v] < d {
            continue;
        }

        for (next, next_d) in &list_yen[v] {
            if d_yen[*next] > d_yen[v] + next_d {
                d_yen[*next] = d_yen[v] + next_d;
                heap.push((*next, d_yen[*next]));
            }
        }
    }

    let mut d_snuke = vec![std::usize::MAX; n];
    let mut heap = BinaryHeap::new();
    heap.push((t - 1, 0));
    d_snuke[t - 1] = 0;

    while let Some((v, d)) = heap.pop() {
        if d_snuke[v] < d {
            continue;
        }

        for (next, next_d) in &list_snuke[v] {
            if d_snuke[*next] > d_snuke[v] + next_d {
                d_snuke[*next] = d_snuke[v] + next_d;
                heap.push((*next, d_snuke[*next]));
            }
        }
    }

    let mut r = vec![0; n];
    for i in (0..n).rev() {
        r[i] = 1_000_000_000_000_000_u64
            .saturating_sub(d_yen[i] as u64)
            .saturating_sub(d_snuke[i] as u64);
        if i < n - 1 {
            r[i] = std::cmp::max(r[i], r[i + 1]);
        }
    }

    for x in r {
        println!("{}", x);
    }
}
