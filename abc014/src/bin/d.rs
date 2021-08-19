use proconio::fastout;
use proconio::input;

fn dfs(
    i: usize,
    d: usize,
    list: &Vec<Vec<usize>>,
    depth: &mut Vec<usize>,
    parents: &mut Vec<Vec<i32>>,
) {
    depth[i] = d;

    for next in &list[i] {
        if depth[*next] < depth.len() {
            continue;
        }
        parents[0][*next] = i as i32;
        dfs(*next, d + 1, list, depth, parents);
    }
}

fn lca(mut i: usize, mut j: usize, parents: &Vec<Vec<i32>>, depth: &Vec<usize>) -> usize {
    if depth[i] < depth[j] {
        std::mem::swap(&mut i, &mut j);
    }

    for k in 0..parents.len() {
        if (depth[i] - depth[j]) >> k & 1 > 0 {
            i = parents[k][i] as usize;
        }
    }

    if i == j {
        return i;
    }

    for k in (0..parents.len()).rev() {
        if parents[k][i] != parents[k][j] {
            i = parents[k][i] as usize;
            j = parents[k][j] as usize;
        }
    }

    parents[0][i] as usize
}

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n - 1],
        q: usize,
        ab: [(usize, usize); q],
    }

    let mut list = vec![vec![]; n];
    for (x, y) in xy {
        list[x - 1].push(y - 1);
        list[y - 1].push(x - 1);
    }

    let mut l = 1;
    while (1 << l) < n {
        l += 1;
    }

    let mut depth = vec![n; n];
    let mut parents = vec![vec![-1; n]; l];

    dfs(0, 0, &list, &mut depth, &mut parents);

    for i in 0..(parents.len() - 1) {
        for j in 0..parents[0].len() {
            if parents[i][j] != -1 {
                parents[i + 1][j] = parents[i][parents[i][j] as usize];
            }
        }
    }

    for (a, b) in ab {
        let p = lca(a - 1, b - 1, &parents, &depth);

        println!("{}", depth[a - 1] + depth[b - 1] - depth[p] * 2 + 1);
    }
}
