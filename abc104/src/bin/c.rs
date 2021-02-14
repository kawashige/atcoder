use proconio::input;

fn dfs(
    problems: &[(usize, usize)],
    i: usize,
    used: &mut Vec<bool>,
    count: usize,
    remains: usize,
    min_count: &mut usize,
) {
    let n = std::cmp::min((remains + i) / (i + 1), problems[i].0);
    let sum = n * (i + 1)
        + if n == problems[i].0 {
            problems[i].1 / 100
        } else {
            0
        };

    if remains <= sum {
        *min_count = std::cmp::min(*min_count, count + n);
    } else if count + n < *min_count {
        for j in 0..problems.len() {
            if used[j] {
                continue;
            }
            used[j] = true;
            dfs(problems, j, used, count + n, remains - sum, min_count);
            used[j] = false;
        }
    }
}

fn main() {
    input! {
        d: usize,
        g: usize,
        problems: [(usize, usize); d],
    }

    let g = g / 100;
    let mut used = vec![false; d];
    let mut min_count = std::usize::MAX;
    for i in 0..d {
        used[i] = true;
        dfs(&problems, i, &mut used, 0, g, &mut min_count);
        used[i] = false;
    }

    println!("{}", min_count);
}
