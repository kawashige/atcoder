use proconio::input;
use std::collections::HashSet;

fn dfs(
    i: usize,
    c: &Vec<usize>,
    lists: &Vec<Vec<usize>>,
    used: &mut Vec<bool>,
    colors: &mut HashSet<usize>,
    results: &mut Vec<usize>,
) {
    let mut is_good = false;
    if !colors.contains(&c[i]) {
        results.push(i + 1);
        colors.insert(c[i]);
        is_good = true;
    }

    for next in &lists[i] {
        if used[*next] {
            continue;
        }
        used[*next] = true;
        dfs(*next, c, lists, used, colors, results);
    }

    if is_good {
        colors.remove(&c[i]);
    }
}

fn main() {
    input! {
        n: usize,
        c: [usize; n],
        ab: [(usize, usize); n - 1]
    }

    let mut lists = vec![vec![]; n];
    for (a, b) in ab {
        lists[a - 1].push(b - 1);
        lists[b - 1].push(a - 1);
    }

    let mut used = vec![false; n];
    used[0] = true;
    let mut colors = HashSet::new();
    let mut results = Vec::new();

    dfs(0, &c, &&lists, &mut used, &mut colors, &mut results);

    results.sort_unstable();

    for i in results {
        println!("{}", i);
    }
}
