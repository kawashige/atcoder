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
        k: usize
    }

    let m = 1_000_000_007;
    let zero_max = std::cmp::min(n - 1, k);

    let mut factorial = vec![0; n + 1];
    factorial[0] = 1;
    factorial[1] = 1;
    for i in 2..factorial.len() {
        factorial[i] = factorial[i - 1] * i % m;
    }

    let mut count = 1;
    for i in 1..=zero_max {
        count += ((((factorial[n] * modinv(factorial[n - i]) % m) * modinv(factorial[i]) % m)
            * factorial[n - 1]
            % m)
            * modinv(factorial[n - 1 - i])
            % m)
            * modinv(factorial[i])
            % m;
        count %= m;
    }

    println!("{}", count);
}
