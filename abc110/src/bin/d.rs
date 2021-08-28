use std::collections::HashMap;

use proconio::input;

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
        n: usize,
        m: usize
    }

    if n == 1 {
        println!("1");
        return;
    }

    let mut x = m;
    let mut i = 2;
    let mut primes = HashMap::new();
    while x >= i {
        while x % i == 0 {
            *primes.entry(i).or_insert(0) += 1;
            x /= i;
        }
        i += 1;
    }

    let mut factorial = vec![1; 2 * n];
    for i in 2..factorial.len() {
        factorial[i] = (factorial[i - 1] * i) % M;
    }

    let mut r = 1;
    for (_, c) in primes {
        r *= (factorial[n + c - 1] * modinv(factorial[n - 1]) % M) * modinv(factorial[c]) % M;
        r %= M;
    }

    println!("{}", r);
}
