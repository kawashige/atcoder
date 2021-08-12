use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        m: usize,
        xy: [(usize, usize); m]
    }

    const M: usize = 1_000_000_007;

    let mut list = vec![vec![]; n];
    for (x, y) in xy {
        list[x - 1].push(y - 1);
        list[y - 1].push(x - 1);
    }

    let mut deque = VecDeque::new();
    deque.push_back(a - 1);

    let mut dist = vec![std::usize::MAX; n];
    let mut count = vec![0; n];

    dist[a - 1] = 0;
    count[a - 1] = 1;

    while let Some(v) = deque.pop_front() {
        for next in &list[v] {
            if dist[*next] >= dist[v] + 1 {
                count[*next] += count[v];
                count[*next] %= M;

                if dist[*next] > dist[v] + 1 {
                    dist[*next] = dist[v] + 1;
                    deque.push_back(*next);
                }
            }
        }
    }

    println!("{}", count[b - 1])
}
