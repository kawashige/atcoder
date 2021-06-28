use proconio::input;

fn dfs(i: usize, list: &Vec<Vec<usize>>, seen: &mut Vec<bool>, visited: usize, count: &mut usize) {
    if visited == seen.len() {
        *count += 1;
        return;
    }

    for next in &list[i] {
        if seen[*next] {
            continue;
        }
        seen[*next] = true;

        dfs(*next, list, seen, visited + 1, count);

        seen[*next] = false;
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut list = vec![vec![]; n];
    for (a, b) in ab {
        list[a - 1].push(b - 1);
        list[b - 1].push(a - 1);
    }

    let mut count = 0;
    let mut seen = vec![false; n];
    seen[0] = true;

    dfs(0, &list, &mut seen, 1, &mut count);

    println!("{}", count);
}
