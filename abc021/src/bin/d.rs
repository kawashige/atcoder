use proconio::input;

fn modinv(a: u64) -> u64 {
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
    u as u64
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    const M: u64 = 1_000_000_007;

    let mut factorial = vec![1; std::cmp::max(n, k) + 2];
    for i in 1..factorial.len() {
        factorial[i] = factorial[i - 1] * i as u64 % M;
    }

    let mut r = 0;
    for i in 1..=std::cmp::min(n, k) {
        r += ((factorial[n] * modinv(factorial[n - i]) % M) * modinv(factorial[i]) % M)
            * ((factorial[k - 1] * modinv(factorial[i - 1]) % M) * modinv(factorial[k - i]) % M)
            % M;
        r %= M;
    }

    println!("{}", r);
}
