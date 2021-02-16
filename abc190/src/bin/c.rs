use proconio::input;

fn dfs(
    seen: &mut Vec<bool>,
    peoples: &[(usize, usize)],
    i: usize,
    conditions: &[(usize, usize)],
    max: &mut usize,
) {
    if peoples.len() - 1 < i {
        let count = conditions
            .iter()
            .filter(|(c1, c2)| seen[*c1] && seen[*c2])
            .count();
        *max = std::cmp::max(*max, count);
        return;
    }

    if !seen[peoples[i].0] {
        seen[peoples[i].0] = true;
        dfs(seen, peoples, i + 1, conditions, max);
        seen[peoples[i].0] = false;
    }
    if !seen[peoples[i].1] {
        seen[peoples[i].1] = true;
        dfs(seen, peoples, i + 1, conditions, max);
        seen[peoples[i].1] = false;
    }
    if !seen[peoples[i].0] && !seen[peoples[i].1] {
        dfs(seen, peoples, i + 1, conditions, max);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        conditions: [(usize, usize); m],
        k: usize,
        peoples: [(usize, usize); k]
    }

    let mut seen = vec![false; n + 1];
    let mut max = 0;
    dfs(&mut seen, &peoples, 0, &conditions, &mut max);

    println!("{}", max);
}
