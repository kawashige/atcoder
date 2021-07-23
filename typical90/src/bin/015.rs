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
        n: usize
    }
    const M: usize = 1_000_000_007;

    let mut factorial = vec![1; n + 2];
    for i in 1..factorial.len() {
        factorial[i] = factorial[i - 1] * i % M;
    }

    for k in 1..=n {
        let mut r = 0;
        for i in 1..=(n / k + 1) {
            if n < (k - 1) * (i - 1) || n - (k - 1) * (i - 1) < i {
                continue;
            }
            r += (factorial[n - (k - 1) * (i - 1)] * modinv(factorial[i]) % M)
                * modinv(factorial[n - (k - 1) * (i - 1) - i])
                % M;
            r %= M
        }
        println!("{}", r);
    }
}
