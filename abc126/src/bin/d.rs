use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        uvw: [(usize, usize, usize); n - 1]
    }

    let mut lists = vec![vec![]; n];
    for (u, v, w) in uvw {
        lists[u - 1].push((v - 1, w));
        lists[v - 1].push((u - 1, w));
    }

    let mut colors = vec![-1; n];
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    while let Some((node, color)) = queue.pop_front() {
        colors[node] = color;

        for (next, w) in &lists[node] {
            if colors[*next] > -1 {
                continue;
            }

            let c = if w % 2 == 0 { color } else { (color + 1) % 2 };
            queue.push_back((*next, c));
        }
    }

    for i in 0..n {
        println!("{}", colors[i]);
    }
}
