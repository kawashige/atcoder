use proconio::input;

fn modinv(a: usize) -> usize {
    let mut a = a as i64;
    let m = 998244353;
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

fn pow(x: usize, n: usize) -> usize {
    if n == 1 {
        x
    } else if n % 2 == 0 {
        let r = pow(x, n / 2);
        r * r % 998244353
    } else {
        x * pow(x, n - 1) % 998244353
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize
    }
    let d = 998244353;

    if m == 1 {
        println!("{}", if k == n - 1 { 1 } else { 0 });
        return;
    }

    if n == 1 {
        println!("{}", m);
        return;
    }

    let mut factorial = vec![0; n];
    factorial[0] = 1;
    for i in 1..n {
        factorial[i] = factorial[i - 1] * i % d;
    }

    let mut count = m * pow(m - 1, n - 1) % d;
    let mut mul = count * factorial[n - 1] % d;
    for i in 1..=k {
        mul = mul * modinv(m - 1) % d;
        count += (mul * modinv(factorial[n - 1 - i]) % d) * modinv(factorial[i]) % d;
        count %= d;
    }

    println!("{}", count);
}
