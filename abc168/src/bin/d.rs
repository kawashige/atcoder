use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        routes: [(usize, usize); m]
    }

    let mut lists = vec![vec![]; n + 1];

    for (a, b) in routes {
        lists[a].push(b);
        lists[b].push(a);
    }

    let mut dist = vec![0; n + 1];
    let mut seen = vec![false; n + 1];
    seen[1] = true;
    let mut queue = VecDeque::new();
    queue.push_front(1_usize);

    while let Some(n) = queue.pop_front() {
        for a in &lists[n] {
            let a = *a;
            if dist[a] == 0 {
                dist[a] = n;
            }
            if seen[a] {
                continue;
            }
            seen[a] = true;
            queue.push_back(a);
        }
    }

    println!("Yes");
    for i in 2..=n {
        println!("{}", dist[i]);
    }
}
