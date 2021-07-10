use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
        cd: [(usize, usize); q],
    }

    let mut list = vec![vec![]; n];
    for (a, b) in ab {
        list[a - 1].push(b - 1);
        list[b - 1].push(a - 1);
    }

    let mut dist = vec![0; n];
    let mut seen = vec![false; n];

    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    while let Some((v, d)) = queue.pop_front() {
        if seen[v] {
            continue;
        }

        seen[v] = true;
        dist[v] = d;

        for next in &list[v] {
            if seen[*next] {
                continue;
            }

            queue.push_back((*next, d + 1));
        }
    }

    for (c, d) in cd {
        let x = (dist[d - 1] as i32 - dist[c - 1] as i32).abs();

        if x % 2 == 0 {
            println!("Town");
        } else {
            println!("Road");
        }
    }
}
