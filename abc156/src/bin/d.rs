use proconio::input;

fn pow(n: u64) -> u64 {
    if n == 0 {
        1
    } else if n % 2 == 0 {
        let r = pow(n / 2);
        r * r % 1_000_000_007
    } else {
        pow(n - 1) * 2 % 1_000_000_007
    }
}

fn product(s: u64, e: u64) -> u64 {
    let mut p = 1;
    for i in s..=e {
        p = p * i % 1_000_000_007;
    }
    p
}

fn modinv(mut a: i64) -> i64 {
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
    u
}

fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64
    }

    let mut sum: i64 = pow(n) as i64
        - 1
        - product(n - a + 1, n) as i64 * modinv(product(1, a) as i64) % 1_000_000_007
        - product(n - b + 1, n) as i64 * modinv(product(1, b) as i64) % 1_000_000_007;
    while sum < 0 {
        sum += 1_000_000_007;
    }

    println!("{}", sum);
}
