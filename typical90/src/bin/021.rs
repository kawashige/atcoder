use proconio::input;

fn dfs(
    i: usize,
    list: &Vec<Vec<usize>>,
    label: &mut Vec<usize>,
    seen: &mut Vec<bool>,
    count: &mut usize,
) {
    seen[i] = true;

    for next in &list[i] {
        if seen[*next] {
            continue;
        }

        dfs(*next, list, label, seen, count);
    }

    label[i] = *count;
    *count += 1;
}

fn dfs2(i: usize, list: &Vec<Vec<usize>>, seen: &mut Vec<bool>, count: &mut usize) {
    seen[i] = true;
    *count += 1;

    for next in &list[i] {
        if seen[*next] {
            continue;
        }

        dfs2(*next, list, seen, count);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut list = vec![vec![]; n];
    let mut rev_list = vec![vec![]; n];

    for (a, b) in ab {
        list[a - 1].push(b - 1);
        rev_list[b - 1].push(a - 1);
    }

    let mut label = vec![0; n];
    let mut count = 0;
    let mut seen = vec![false; n];

    for i in 0..n {
        if !seen[i] {
            dfs(i, &list, &mut label, &mut seen, &mut count);
        }
    }

    let mut seen = vec![false; n];
    let mut target = label
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, usize)>>();
    target.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let mut r = 0;
    for (i, _) in target {
        if !seen[i] {
            let mut count = 0;
            dfs2(i, &rev_list, &mut seen, &mut count);
            if count > 1 {
                r += (count - 1) * count / 2;
            }
        }
    }

    println!("{}", r);
}
