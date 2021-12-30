use proconio::input;

fn dfs(p: &mut Vec<usize>, n: usize, ab: &Vec<(usize, usize)>, cd: &Vec<(usize, usize)>) -> bool {
    if p.len() == n {
        let mut new_cd = cd
            .iter()
            .map(|(c, d)| (p[*c].min(p[*d]), p[*c].max(p[*d])))
            .collect::<Vec<_>>();
        new_cd.sort_unstable();
        if (0..ab.len()).all(|i| ab[i] == new_cd[i]) {
            return true;
        } else {
            return false;
        }
    }

    for i in 0..n {
        if !p.contains(&i) {
            p.push(i);
            if dfs(p, n, ab, cd) {
                return true;
            }
            p.pop();
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); m]
    }

    let mut ab = ab
        .into_iter()
        .map(|(a, b)| ((a - 1).min(b - 1), (a - 1).max(b - 1)))
        .collect::<Vec<_>>();
    ab.sort_unstable();
    let cd = cd
        .into_iter()
        .map(|(a, b)| ((a - 1).min(b - 1), (a - 1).max(b - 1)))
        .collect::<Vec<_>>();

    if dfs(&mut Vec::new(), n, &ab, &cd) {
        println!("Yes");
    } else {
        println!("No");
    }
}
