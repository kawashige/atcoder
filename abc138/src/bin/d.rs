use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        q: usize,
        edges: [(usize, usize); n - 1],
        query: [(usize, usize); q]
    }

    let mut v = vec![0; n];
    for (p, x) in query {
        v[p - 1] += x;
    }

    let mut e = vec![vec![]; n];
    for (f, t) in edges {
        e[f - 1].push(t - 1);
        e[t - 1].push(f - 1);
    }

    let mut queue = VecDeque::new();
    queue.push_back((0, v[0]));

    let mut p = vec![0; n];
    let mut seen = vec![false; n];
    while let Some((node, point)) = queue.pop_front() {
        p[node] = point;
        seen[node] = true;
        for edge in &e[node] {
            if !seen[*edge] {
                queue.push_back((*edge, point + v[*edge]));
            }
        }
    }

    println!(
        "{}",
        p.into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
