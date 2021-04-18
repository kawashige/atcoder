use proconio::input;

fn dfs(a: &Vec<u64>, sum_a: &Vec<u64>, i: usize, sum: u64, t: u64, max: &mut u64) {
    if sum > t || *max == t {
        return;
    }

    if sum_a[i] + sum <= t {
        *max = std::cmp::max(*max, sum_a[i] + sum);
        return;
    }

    if i == 0 {
        *max = std::cmp::max(*max, sum);
        return;
    }

    dfs(a, sum_a, i - 1, sum + a[i], t, max);
    dfs(a, sum_a, i - 1, sum, t, max);
}

fn main() {
    input! {
        n: usize,
        t: u64,
        mut a: [u64; n]
    }

    a.sort_unstable();

    let mut sum_a = vec![0; n];
    sum_a[0] = a[0];
    for i in 1..n {
        sum_a[i] = a[i] + sum_a[i - 1];
    }

    if sum_a[n - 1] <= t {
        println!("{}", sum_a[n - 1]);
        return;
    }

    let mut max = 0;

    dfs(&a, &sum_a, n - 1, 0, t, &mut max);

    println!("{}", max);
}
