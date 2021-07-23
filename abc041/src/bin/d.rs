use proconio::input;

fn dfs(
    free_count: u64,
    prev_count: &mut Vec<usize>,
    succ: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
) -> u64 {
    let mut r = 0;

    for i in 0..seen.len() {
        if seen[i] || prev_count[i] > 0 {
            continue;
        }

        seen[i] = true;
        for j in &succ[i] {
            prev_count[*j] -= 1;
        }

        r += dfs(free_count, prev_count, succ, seen);

        seen[i] = false;
        for j in &succ[i] {
            prev_count[*j] += 1;
        }
    }

    r.max(1)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(usize, usize); m]
    }

    let mut succ = vec![vec![]; n];
    let mut prev_count = vec![0; n];
    for (x, y) in xy {
        succ[x - 1].push(y - 1);
        prev_count[y - 1] += 1;
    }

    let mut seen = vec![false; n];
    let mut free_count = 0;
    for i in 0..n {
        if succ[i].is_empty() && prev_count[i] == 0 {
            free_count += 1;
            seen[i] = true;
        }
    }

    let mut factorial = vec![0; n + 3];
    factorial[0] = 1;
    for i in 1..factorial.len() {
        factorial[i] = factorial[i - 1] * i as u64
    }

    let r = dfs(free_count as u64, &mut prev_count, &succ, &mut seen)
        * if free_count > 0 {
            factorial[free_count]
                * (factorial[n] / (factorial[n - free_count] * factorial[free_count]))
        } else {
            1
        };

    println!("{}", r);
}
