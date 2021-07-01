use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m]
    }

    let mut used = vec![false; m];
    let mut list = vec![vec![]; n];

    for (i, (a, b, c)) in abc.into_iter().enumerate() {
        list[a - 1].push((b - 1, c, i));
        list[b - 1].push((a - 1, c, i));
    }

    for x in 0..(n - 1) {
        let mut edge = vec![m; n];
        let mut dist = vec![std::usize::MAX; n];
        dist[x] = 0;
        let mut heap = BinaryHeap::new();
        heap.push((x, 0, None));

        while let Some((v, d, i)) = heap.pop() {
            if d > dist[v] {
                continue;
            }

            if i.is_some() {
                edge[v] = i.unwrap();
            }

            for (next, c, j) in &list[v] {
                if dist[v] != std::usize::MAX && dist[*next] > dist[v] + c {
                    dist[*next] = dist[v] + c;
                    heap.push((*next, dist[*next], Some(*j)));
                }
            }
        }

        for e in edge {
            if e != m {
                used[e] = true;
            }
        }
    }

    let r = used.into_iter().filter(|x| !x).count();
    println!("{}", r);
}
