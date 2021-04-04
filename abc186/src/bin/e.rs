use proconio::input;

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn modinv(mut a: i64, m: i64) -> i64 {
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
    u
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            mut n: i64,
            mut s: i64,
            mut k: i64
        }
        
        let d = gcd(n, gcd(s, k));
        n /= d;
        s /= d;
        k /= d;
        if gcd(n, k) == 1 {
            let mut r = (-s * modinv(k, n)) % n;
            while r < 0 {
                r += n;
            }
            println!("{}", r);
        } else {
            println!("-1");
        }
}
}
