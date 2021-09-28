use proconio::input;

fn dfs1(
    node: usize,
    parent: usize,
    list: &Vec<Vec<usize>>,
    count: &mut Vec<usize>,
    dp: &mut Vec<usize>,
) {
    for child in &list[node] {
        if child == &parent {
            continue;
        }
        dfs1(*child, node, list, count, dp);
        count[node] += count[*child] + 1;
        dp[node] += dp[*child] + count[*child] + 1
    }
}

fn dfs2(
    node: usize,
    parent: usize,
    list: &Vec<Vec<usize>>,
    count: &Vec<usize>,
    dp: &mut Vec<usize>,
) {
    if node != 0 {
        dp[node] = dp[parent] + dp.len() - (count[node] + 1) * 2;
    }

    for child in &list[node] {
        if child == &parent {
            continue;
        }
        dfs2(*child, node, list, count, dp);
    }
}

fn main() {
    input! {
        n: usize,
        uv: [(usize, usize); n - 1]
    }

    let mut list = vec![vec![]; n];
    for (u, v) in uv {
        list[u - 1].push(v - 1);
        list[v - 1].push(u - 1);
    }

    let mut dp = vec![0; n];
    let mut count = vec![0; n];
    dfs1(0, n, &list, &mut count, &mut dp);
    dfs2(0, n, &list, &count, &mut dp);

    for x in dp {
        println!("{}", x);
    }
}
