use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        e: [(usize, usize); n - 1],
        q: usize,
        queries: [[i64; 3]; q],
    }

    let mut depth = vec![-1; n];
    depth[0] = 0;
    let mut edges = vec![vec![]; n];

    for (from, to) in &e {
        edges[from - 1].push(to - 1);
        edges[to - 1].push(from - 1);
    }

    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(n) = queue.pop_front() {
        for to in &edges[n] {
            if depth[*to] == -1 {
                depth[*to] = depth[n] + 1;
                queue.push_back(*to);
            }
        }
    }

    let mut s = vec![0_i64; n];

    for query in queries {
        let (mut a, mut b) = e[query[1] as usize - 1];
        if query[0] == 2 {
            std::mem::swap(&mut a, &mut b);
        }

        if depth[a - 1] < depth[b - 1] {
            s[0] += query[2];
            s[b - 1] -= query[2]
        } else {
            s[a - 1] += query[2];
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(n) = queue.pop_front() {
        for to in &edges[n] {
            if depth[*to] > depth[n] {
                s[*to] += s[n];
                queue.push_back(*to);
            }
        }
    }

    for ss in s {
        println!("{}", ss);
    }
}
