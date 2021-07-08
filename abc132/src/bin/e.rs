use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        v: [(usize, usize); m],
        s: usize,
        t: usize
    }

    let mut list = vec![vec![]; n];

    for (from, to) in v {
        list[from - 1].push(to - 1);
    }

    let mut queue = VecDeque::new();
    queue.push_back((s - 1, 0));
    let mut seen = vec![false; n];
    let mut seen1 = vec![false; n];
    let mut seen2 = vec![false; n];

    while let Some((node, d)) = queue.pop_front() {
        if node == t - 1 {
            println!("{}", d);
            return;
        }

        seen[node] = true;

        for next1 in &list[node] {
            if seen1[*next1] {
                continue;
            }
            seen1[*next1] = true;
            for next2 in &list[*next1] {
                if seen2[*next2] {
                    continue;
                }
                seen2[*next2] = true;
                for next3 in &list[*next2] {
                    if seen[*next3] {
                        continue;
                    }
                    queue.push_back((*next3, d + 1))
                }
            }
        }
    }

    println!("-1");
}
