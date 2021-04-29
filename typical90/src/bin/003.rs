use proconio::input;

fn dfs(lists: &Vec<Vec<usize>>, parent: usize, child: usize, max: &mut usize) -> usize {
    let mut d = lists[child]
        .iter()
        .filter(|l| l != &&parent)
        .map(|i| dfs(lists, child, *i, max) + 1)
        .collect::<Vec<usize>>();

    if d.is_empty() {
        return 0;
    }

    d.sort_unstable_by(|a, b| b.cmp(&a));

    if d.len() > 1 {
        *max = std::cmp::max(*max, d[0] + d[1]);
    }
    *max = std::cmp::max(*max, d[0]);
    d[0]
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1]
    }

    let mut lists = vec![vec![]; n];
    for (a, b) in ab {
        lists[a - 1].push(b - 1);
        lists[b - 1].push(a - 1);
    }
    let mut max = 0;

    dfs(&lists, n, 0, &mut max);

    println!("{}", max + 1);
}
