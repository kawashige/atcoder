use proconio::input;

fn dfs(i: usize, parent: usize, list: &Vec<Vec<usize>>, seen: &mut Vec<bool>, is_tree: &mut bool) {
    seen[i] = true;

    for next in &list[i] {
        if next == &parent {
            continue;
        }

        if seen[*next] {
            *is_tree = false;
            continue;
        }

        dfs(*next, i, list, seen, is_tree);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m]
    }

    let mut list = vec![vec![]; n];
    for (u, v) in uv {
        list[u - 1].push(v - 1);
        list[v - 1].push(u - 1);
    }

    let mut r = 0;
    let mut seen = vec![false; n];
    for i in 0..n {
        if seen[i] {
            continue;
        }

        let mut is_tree = true;

        dfs(i, n, &list, &mut seen, &mut is_tree);

        if is_tree {
            r += 1;
        }
    }

    println!("{}", r);
}
