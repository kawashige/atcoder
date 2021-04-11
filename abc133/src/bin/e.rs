use proconio::input;

fn modinv(a: usize) -> usize {
    let mut a = a as i64;
    let m = 1_000_000_007;
    let mut b = m;
    let mut u = 1;
    let mut v = 0;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        std::mem::swap(&mut a, &mut b);
        u -= t * v;
        std::mem::swap(&mut u, &mut v);
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    u as usize
}

fn main() {
    input! {
        n: usize,
        k: usize,
        edges: [(usize, usize); n - 1]
    }

    let mut lists = vec![vec![]; n];
    for (p, c) in edges {
        lists[p - 1].push(c - 1);
        lists[c - 1].push(p - 1);
    }

    let mut min_colors = 0;
    let mut stack = vec![0];
    let mut seen = vec![false; n];
    while let Some(node) = stack.pop() {
        seen[node] = true;

        let mut tmp = lists[node].len() + if node == 0 { 1 } else { 0 };
        for child in &lists[node] {
            if seen[*child] {
                continue;
            }
            tmp = std::cmp::max(tmp, lists[*child].len() + 1);
            stack.push(*child);
        }

        min_colors = std::cmp::max(min_colors, tmp);
    }

    println!("{}", min_colors);

    if min_colors > k {
        println!("0");
        return;
    }

    let max_colors = std::cmp::min(n, k);

    let mut factorials = vec![1; k + 1];
    for i in 2..factorials.len() {
        factorials[i] = factorials[i - 1] * i;
    }
    println!("{}, {}", min_colors, max_colors);

    let mut sum = 0;
    for i in min_colors..=max_colors {
        sum = (sum
            + (factorials[k] * modinv(factorials[k - i]) % 1_000_000_007) * modinv(factorials[i])
                % 1_000_000_007)
            % 1_000_000_007;
    }

    println!("{}", sum);
}
