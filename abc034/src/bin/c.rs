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
        w: usize,
        h: usize
    }

    const M: usize = 1_000_000_007;

    let mut factorial = vec![0; w + h];
    factorial[0] = 1;
    for i in 1..factorial.len() {
        factorial[i] = i * factorial[i - 1] % M;
    }

    let r = (factorial[w + h - 2] * modinv(factorial[w - 1]) % M) * modinv(factorial[h - 1]) % M;

    println!("{}", r);
}
