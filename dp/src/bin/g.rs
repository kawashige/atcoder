use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m]
    }

    let mut lists = vec![vec![]; n];
    let mut from_count = vec![0; n];

    for (x, y) in edges {
        lists[x - 1].push(y - 1);
        from_count[y - 1] += 1;
    }

    let mut paths = vec![0; n];
    let mut queue = VecDeque::new();

    for i in 0..n {
        if from_count[i] == 0 {
            queue.push_back(i);
        }
    }

    while let Some(node) = queue.pop_front() {
        for child in &lists[node] {
            paths[*child] = std::cmp::max(paths[*child], paths[node] + 1);
            from_count[*child] -= 1;
            if from_count[*child] == 0 {
                queue.push_back(*child);
            }
        }
    }

    println!("{}", paths.iter().max().unwrap());
}
