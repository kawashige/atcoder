use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        lrd: [(usize, usize, usize); m],
    }

    if m == 0 {
        println!("Yes");
        return;
    }

    let mut childs = vec![vec![]; n];
    let mut from = vec![0; n];
    let mut to = vec![0; n];
    for (l, r, d) in lrd {
        childs[l - 1].push((r - 1, d));
        from[r - 1] += 1;
        to[l - 1] += 1;
    }

    let mut dist: Vec<HashMap<usize, usize>> = vec![HashMap::new(); n];
    let mut stack = Vec::new();

    for i in 0..n {
        if from[i] == 0 && to[i] != 0 {
            stack.push(i);
            dist[i] = vec![(i, 0)].into_iter().collect();
        }
    }

    if stack.is_empty() {
        println!("No");
        return;
    }

    while let Some(i) = stack.pop() {
        let target = dist[i].clone();
        for (c, c_d) in &childs[i] {
            for (s, d) in &target {
                if let Some(x) = dist[*c].get(s) {
                    if *x != d + c_d {
                        println!("No");
                        return;
                    }
                } else {
                    dist[*c].insert(*s, d + c_d);
                }
            }
            from[*c] -= 1;
            if from[*c] == 0 {
                stack.push(*c);
            }
        }
    }
    println!("Yes");
}
