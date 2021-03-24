use proconio::input;

fn recurse(s: usize, v: usize, cost: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>) -> i32 {
    if dp[s][v] != -1 {
        return dp[s][v];
    }

    if s == (1 << v) {
        dp[s][v] = cost[0][v];
        return cost[0][v];
    }

    let mut r = std::i32::MAX;
    let prev = s & !(1 << v);

    for u in 0..dp[0].len() {
        if prev & 1 << u == 0 {
            continue;
        }

        r = std::cmp::min(r, recurse(prev, u, cost, dp) + cost[u][v]);
    }
    dp[s][v] = r;
    r
}

fn main() {
    input! {
        n: usize,
        p: [(i32, i32, i32); n],
    }

    let mut cost = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let d = (p[i].0 - p[j].0).abs() + (p[i].1 - p[j].1).abs();
            cost[i][j] = d + std::cmp::max(0, p[j].2 - p[i].2);
            cost[j][i] = d + std::cmp::max(0, p[i].2 - p[j].2);
        }
    }

    let mut dp = vec![vec![-1; n]; (1 << n) + 1];
    let r = recurse((1 << n) - 1, 0, &cost, &mut dp);

    println!("{}", r);
}
