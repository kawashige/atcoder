use proconio::input;

fn dfs(
    times: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    k: usize,
    prev: usize,
    count: &mut usize,
    time: usize,
) {
    if seen.iter().all(|n| *n) {
        if time + times[prev][0] == k {
            *count += 1;
        }
        return;
    }

    if k <= time {
        return;
    }

    for i in 1..seen.len() {
        if seen[i] {
            continue;
        }
        seen[i] = true;
        dfs(times, seen, k, i, count, time + times[prev][i]);
        seen[i] = false;
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        times: [[usize; n]; n]
    }

    let mut seen = vec![false; n];
    seen[0] = true;
    let mut count = 0;
    dfs(&times, &mut seen, k, 0, &mut count, 0);
    println!("{}", count);
}
