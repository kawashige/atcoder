use proconio::input;
use proconio::marker::Chars;

fn dfs(
    map: &mut Vec<Vec<char>>,
    i: usize,
    j: usize,
    k: usize,
    vw: &Vec<(Vec<usize>, Vec<char>)>,
) -> bool {
    if i == vw.len() {
        return true;
    }

    if j == vw[i].0.len() && k == vw[i].1.len() {
        return dfs(map, i + 1, 0, 0, vw);
    } else if j == vw[i].0.len() || k == vw[i].1.len() {
        return false;
    }

    if map[vw[i].0[j]].is_empty() {
        for l in k..(k + 3).min(vw[i].1.len() - (vw[i].0.len() - j - 1)) {
            map[vw[i].0[j]] = vw[i].1[k..=l].to_vec();
            if dfs(map, i, j + 1, l + 1, vw) {
                return true;
            }
        }
        map[vw[i].0[j]] = vec![];
        false
    } else if vw[i].1[k..].starts_with(&map[vw[i].0[j]]) {
        dfs(map, i, j + 1, k + map[vw[i].0[j]].len(), vw)
    } else {
        false
    }
}

fn main() {
    input! {
        k: usize,
        n: usize,
        vw: [(Chars, Chars); n]
    }

    let vw = vw
        .into_iter()
        .map(|(v, w)| {
            (
                v.into_iter()
                    .map(|x| x.to_digit(10).unwrap() as usize)
                    .collect::<Vec<_>>(),
                w,
            )
        })
        .collect::<Vec<_>>();

    let mut result = vec![vec![]; k + 1];
    dfs(&mut result, 0, 0, 0, &vw);

    for i in 1..=k {
        println!("{}", result[i].iter().collect::<String>());
    }
}
