use proconio::input;

fn modinv(a: i64) -> i64 {
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
    u as i64
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: usize,
        b: usize
    }

    const M: i64 = 1_000_000_007;

    let mut factorial = vec![0; h + w + 1];
    factorial[0] = 1;
    factorial[1] = 1;
    for i in 2..factorial.len() {
        factorial[i] = i as i64 * factorial[i - 1] % M;
    }

    let mut sum =
        (factorial[h - 1 + w - 1] * modinv(factorial[w - 1]) % M) * modinv(factorial[h - 1]) % M;

    for i in 0..a {
        let mut count = (factorial[h - 1 - i + b - 1] * modinv(factorial[h - 1 - i]) % M)
            * modinv(factorial[b - 1])
            % M;
        count *= (factorial[i + w - b - 1] * modinv(factorial[w - b - 1]) % M)
            * modinv(factorial[i])
            % M;
        count %= M;
        sum -= count;
        if sum < 0 {
            sum += M;
        }
    }

    println!("{}", sum);
}
