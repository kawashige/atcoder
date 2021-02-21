use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        y: usize,
        trains: [(usize, usize, i64, i64); m]
    }

    let mut list = vec![vec![]; n + 1];

    for (a, b, t, k) in trains {
        list[a].push((b, t, k));
        list[b].push((a, t, k));
    }

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), x));

    let mut times = vec![None; n + 1];

    while let Some((Reverse(d), u)) = queue.pop() {
        if times[u].is_some() {
            continue;
        }
        times[u] = Some(d);

        for &(v, t, k) in list[u].iter() {
            queue.push((Reverse((d + k - 1) / k * k + t), v))
        }
    }

    println!("{}", times[y].unwrap_or(-1))
}
