use proconio::input;
use proconio::marker::Chars;

const M: usize = 1_000_000_007;

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
        k: usize,
        s: Chars
    }

    let n = s.len();
    let mut factorial = vec![1; k + n + 1];
    let mut mod_factorial = vec![1; k + n + 1];
    for i in 2..factorial.len() {
        factorial[i] = (factorial[i - 1] * i) % M;
        mod_factorial[i] = modinv(factorial[i]);
    }

    let mut pow25 = vec![1; k + 1];
    let mut pow26 = vec![1; k + 1];
    for i in 1..=k {
        pow25[i] = pow25[i - 1] * 25 % M;
        pow26[i] = pow26[i - 1] * 26 % M;
    }

    let mut r = 0;

    for i in 0..=k {
        let mut c = (factorial[n + i - 1] * mod_factorial[n - 1] % M) * mod_factorial[i] % M;
        c *= pow25[i] * pow26[k - i] % M;
        r += c % M;
        r %= M
    }

    println!("{}", r);
}
