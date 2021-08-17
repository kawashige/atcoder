use proconio::input;

fn dfs(i: usize, next: &Vec<usize>, f: &Vec<usize>, memo: &mut Vec<i64>) -> i64 {
    const M: i64 = 1_000_000_007;

    if i == f.len() || i == f.len() - 1 {
        return 1;
    }

    if memo[i] > 0 {
        return memo[i];
    }

    let mut r = 0;
    if i + 1 < next.len() && next[i] == next[i + 1] {
        r += 2 * dfs(i + 1, next, f, memo) % M;
        r %= M;
    } else {
        for j in i..next[i] {
            r += dfs(j + 1, next, f, memo) % M;
            r %= M;
        }
    }

    memo[i] = r;
    r
}

fn main() {
    input! {
        n: usize,
        m: usize,
        f: [usize; n],
    }

    let mut next = vec![n; n];
    let mut index = vec![n; m + 1];
    for i in (0..n).rev() {
        if index[f[i]] != n {
            next[i] = index[f[i]]
        }
        index[f[i]] = i;
    }
    for i in (0..(n - 1)).rev() {
        next[i] = next[i].min(next[i + 1]);
    }

    let r = dfs(0, &next, &f, &mut vec![-1; n]);

    println!("{}", r);
}
