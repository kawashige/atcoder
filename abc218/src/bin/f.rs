use std::collections::HashSet;

use proconio::input;

fn dfs(
    i: usize,
    list: &Vec<Vec<(usize, usize)>>,
    seen: &mut Vec<bool>,
    path: &mut HashSet<usize>,
    dist: &mut Vec<i32>,
) {
    if i == list.len() - 1 {
        for j in 0..dist.len() {
            if !path.contains(&j) {
                dist[j] = dist[j].min(path.len() as i32);
            }
        }
        return;
    }

    for (next, j) in &list[i] {
        if seen[*next] {
            continue;
        }

        seen[*next] = true;
        path.insert(*j);

        dfs(*next, list, seen, path, dist);

        seen[*next] = false;
        path.remove(j);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        st: [(usize, usize); m]
    }

    let mut list = vec![vec![]; n];
    for i in 0..m {
        list[st[i].0 - 1].push((st[i].1 - 1, i));
    }

    let mut dist = vec![std::i32::MAX; m];
    let mut seen = vec![false; n];
    seen[0] = true;

    dfs(
        0,
        &list,
        &mut vec![false; n],
        &mut HashSet::new(),
        &mut dist,
    );

    for x in dist {
        println!("{}", if x == std::i32::MAX { -1 } else { x });
    }
}
