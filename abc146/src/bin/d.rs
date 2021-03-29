use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        e: [(usize, usize); n - 1],
    }

    let mut edges = vec![vec![]; n];
    for (i, (a, b)) in e.iter().enumerate() {
        edges[a - 1].push((b - 1, i + 1));
        edges[b - 1].push((a - 1, i + 1));
    }

    let mut color = 0;
    let mut colors = vec![0; n];
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    while let Some((child, parent)) = queue.pop_front() {
        let mut j = 1;
        if j == colors[parent] {
            j += 1;
        }

        for e in &edges[child] {
            if colors[e.1] == 0 {
                colors[e.1] = j;
                j += 1;
                if j == colors[parent] {
                    j += 1;
                }
                queue.push_back(*e);
            }
        }

        color = std::cmp::max(color, j - 1);
    }

    println!("{}", color);
    for i in 1..n {
        println!("{}", colors[i]);
    }
}
