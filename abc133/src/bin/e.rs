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

    let m = 1_000_000_007;
    let mut lists = vec![vec![]; n];
    for (p, c) in edges {
        lists[p - 1].push(c - 1);
        lists[c - 1].push(p - 1);
    }

    let mut factorial = vec![0; std::cmp::max(n, k) + 1];
    factorial[0] = 1;
    factorial[1] = 1;
    for i in 2..factorial.len() {
        factorial[i] = i * factorial[i - 1] % m;
    }

    let mut depth = vec![std::usize::MAX; n];
    depth[0] = 0;

    let mut stack = vec![0];
    let mut count = k;
    while let Some(n) = stack.pop() {
        if lists[n].len() + 1 > k {
            println!("0");
            return;
        }

        let c = lists[n].len() - if depth[n] == 0 { 0 } else { 1 };
        if c > 0 {
            let r = k - if depth[n] == 0 { 1 } else { 2 };

            count *= factorial[r] * modinv(factorial[r - c]) % m;
            count %= m;
        }

        for c in &lists[n] {
            if depth[*c] != std::usize::MAX {
                continue;
            }
            depth[*c] = depth[n] + 1;
            stack.push(*c);
        }
    }

    println!("{}", count);
}
