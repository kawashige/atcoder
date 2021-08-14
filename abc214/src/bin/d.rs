use proconio::input;

fn dfs(i: usize, list: &Vec<Vec<(usize, u64)>>, seen: &mut Vec<bool>, w: u64) -> usize {
    seen[i] = true;
    let mut count = 1;
    for (child, weight) in &list[i] {
        if seen[*child] || weight > &w {
            continue;
        }
        count += dfs(*child, list, seen, w);
    }
    count
}

fn main() {
    input! {
        n: usize,
        mut uvw: [(usize, usize, u64); n - 1]
    }

    uvw.sort_unstable_by(|a, b| b.2.cmp(&a.2));

    let mut list = vec![vec![]; n];
    for (u, v, w) in &uvw {
        list[u - 1].push((v - 1, *w));
        list[v - 1].push((u - 1, *w));
    }

    let mut r = 0;
    for (a, b, w) in uvw {
        let mut seen = vec![false; n];
        if seen[a - 1] || seen[b - 1] {
            continue;
        }
        let c = dfs(a - 1, &list, &mut seen, w);

        r += (c - 1) as u64 * w;
    }

    println!("{}", r);
}
