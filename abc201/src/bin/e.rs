use proconio::input;

fn dfs(node: usize, list: &Vec<Vec<usize>>, seen: &mut Vec<bool>, counts: &mut Vec<usize>) {
    let mut count = 0;
    seen[node] = true;
    for next in &list[node] {
        if seen[*next] {
            continue;
        }
        dfs(*next, list, seen, counts);
        count += counts[*next] + 1;
    }
    counts[node] = count;
}

fn main() {
    input! {
        n: usize,
        uvw: [(usize, usize, usize); n - 1]
    }
    let mut list = vec![vec![]; n];
    for (u, v, _) in &uvw {
        list[u - 1].push(v - 1);
        list[v - 1].push(u - 1);
    }

    let mut seen = vec![false; n];
    let mut counts = vec![0; n];
    dfs(0, &list, &mut seen, &mut counts);

    // println!("{:?}", counts);
}
