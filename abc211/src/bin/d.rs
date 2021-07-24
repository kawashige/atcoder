use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    const M: usize = 1_000_000_007;

    let mut list = vec![vec![]; n];
    for (a, b) in ab {
        list[a - 1].push(b - 1);
        list[b - 1].push(a - 1);
    }

    let mut heap = VecDeque::new();
    let mut dist = vec![std::usize::MAX; n];
    let mut count = vec![0; n];
    count[0] = 1;
    dist[0] = 0;
    heap.push_back((0, 0));

    while let Some((d, v)) = heap.pop_front() {
        if dist[v] < d {
            continue;
        }

        for next in &list[v] {
            if dist[*next] > dist[v] + 1 {
                dist[*next] = dist[v] + 1;
                heap.push_back((dist[*next], *next));
                count[*next] = count[v];
            } else if dist[*next] == dist[v] + 1 {
                count[*next] += count[v];
                count[*next] %= M;
            }
        }
    }

    println!("{}", count[n - 1]);
}
