use proconio::input;
use std::collections::VecDeque;

fn recurse(
    s: usize,
    v: usize,
    g: usize,
    cost: &Vec<Vec<i32>>,
    dp: &mut Vec<Vec<i32>>,
    c: &Vec<usize>,
) -> i32 {
    if dp[s][v] != -1 {
        return dp[s][v];
    }

    let mut r = std::i32::MAX;
    let prev = s & !(1 << v);

    if prev == 0 {
        return 0;
    }

    for u in 0..dp[0].len() {
        if prev & 1 << u == 0 {
            continue;
        }

        r = std::cmp::min(
            r,
            recurse(prev, u, g, cost, dp, c).saturating_add(cost[u][c[v] - 1]),
        );
    }

    dp[s][v] = r;
    r
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        k: usize,
        c: [usize; k],
    }
    if k == 1 {
        println!("{}", 1);
        return;
    }

    let mut list = vec![vec![]; n];
    for (a, b) in ab {
        list[a - 1].push(b - 1);
        list[b - 1].push(a - 1);
    }

    let mut dist = vec![vec![std::i32::MAX; n]; k];

    for i in 0..k {
        let mut queue = VecDeque::new();
        queue.push_back((0, c[i] - 1));

        while let Some((d, v)) = queue.pop_front() {
            if dist[i][v] != std::i32::MAX {
                continue;
            }

            dist[i][v] = d;

            for next in &list[v] {
                if dist[i][*next] != std::i32::MAX {
                    continue;
                }
                queue.push_back((d + 1, *next))
            }
        }
    }

    let mut dp = vec![vec![-1; k]; (1 << k) + 1];
    let mut r = std::i32::MAX;
    for i in 0..k {
        let tmp = recurse((1 << k) - 1, i, i, &dist, &mut dp, &c);
        r = std::cmp::min(r, tmp);
    }

    if r == std::i32::MAX {
        println!("-1");
    } else {
        println!("{}", r + 1);
    }
}
