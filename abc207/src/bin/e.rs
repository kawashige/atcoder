use proconio::input;

fn dfs(
    i: usize,
    k: usize,
    a: &Vec<u64>,
    mods: &Vec<Vec<Vec<usize>>>,
    memo: &mut Vec<Vec<i32>>,
) -> i32 {
    if memo[i][k] > 0 {
        return memo[i][k];
    }

    if i + 1 < k {
        return 0;
    }

    let mut count = 0;
    for j in &mods[i][k] {
        if j == &0 {
            continue;
        }
        count += dfs(j - 1, k - 1, a, mods, memo);
        count %= 1_000_000_007;
    }

    memo[i][k] = count;
    count
}

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }

    let mut memo = vec![vec![-1; n + 1]; n];
    for i in 0..n {
        memo[i][1] = 1;
    }

    let mut mods = vec![vec![vec![]; n + 1]; n];
    let mut sums = vec![0; n];

    for i in 0..n {
        for j in 0..=i {
            sums[j] += a[i];
            for k in 1..=(i + 1) {
                if sums[j] % k as u64 == 0 {
                    mods[i][k].push(j);
                } else {
                    memo[i][k] = 0;
                }
            }
        }
    }

    let mut r = 1;
    for k in (2..=n).rev() {
        r += dfs(n - 1, k, &a, &mods, &mut memo);
        r %= 1_000_000_007;
    }

    println!("{}", r);
}
