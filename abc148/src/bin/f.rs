use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        u: usize,
        v: usize,
        ab: [(usize, usize); n - 1]
    }

    let mut lists = vec![vec![]; n];
    for (a, b) in ab {
        lists[a - 1].push(b - 1);
        lists[b - 1].push(a - 1);
    }

    let mut t = vec![-1; n];
    let mut a = vec![-1; n];

    let mut queue = VecDeque::new();
    queue.push_back((u - 1, 0));
    while let Some((node, time)) = queue.pop_front() {
        if t[node] > -1 {
            continue;
        }

        t[node] = time;
        for next in &lists[node] {
            queue.push_back((*next, time + 1));
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((v - 1, 0));
    while let Some((node, time)) = queue.pop_front() {
        if a[node] > -1 {
            continue;
        }

        a[node] = time;
        for next in &lists[node] {
            queue.push_back((*next, time + 1));
        }
    }

    let r = (0..n)
        .filter(|i| t[*i] < a[*i])
        .map(|i| a[i] - 1)
        .max()
        .unwrap();

    println!("{}", r);
}
