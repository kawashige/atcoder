use proconio::input;

fn dfs(x: usize, n: usize, d: &Vec<usize>, v: &mut Vec<usize>) {
    if n <= x {
        v.push(x);
        return;
    }

    for i in 0..d.len() {
        dfs(x * 10 + d[i], n, d, v);
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        d: [usize; k]
    }

    let d = (0..10).filter(|i| !d.contains(i)).collect::<Vec<usize>>();
    let mut v = Vec::new();

    for i in &d {
        if i == &0 {
            continue;
        }
        dfs(*i, n, &d, &mut v);
    }
    v.sort_unstable();

    println!("{}", v[0]);
}
