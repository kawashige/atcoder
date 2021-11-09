use proconio::input;

fn dfs(
    i: usize,
    list: &Vec<Vec<(usize, usize)>>,
    seen: &mut Vec<bool>,
    used: &mut Vec<bool>,
    cycle: &mut usize,
) {
    seen[i] = true;
    for (next, j) in &list[i] {
        if used[*j] {
            continue;
        }
        used[*j] = true;
        if seen[*next] {
            *cycle += 1;
            continue;
        }
        dfs(*next, list, seen, used, cycle);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m]
    }

    let mut list = vec![vec![]; n];
    for (i, (u, v)) in uv.into_iter().enumerate() {
        list[u - 1].push((v - 1, i));
        list[v - 1].push((u - 1, i));
    }

    let mut r = 1;
    let mut seen = vec![false; n];
    let mut used = vec![false; m];
    for i in 0..n {
        if seen[i] {
            continue;
        }

        let mut cycle = 0;

        dfs(i, &list, &mut seen, &mut used, &mut cycle);

        if cycle == 1 {
            r *= 2;
            r %= 998244353;
        } else {
            println!("0");
            return;
        }
    }
    println!("{}", r);
}
