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
        m: usize,
        k: usize
    }

    let modulo: u64 = 1_000_000_007;

    let mut factorial = vec![0; n + m + 1];
    factorial[0] = 1;
    factorial[1] = 1;
    for i in 2..=(n + m) {
        factorial[i] = factorial[i - 1] * i as u64 % modulo;
    }

    let mut sum =
        ((factorial[n + m] * modinv(factorial[n]) % modulo) * modinv(factorial[m]) % modulo) as i64;

    let mut div = 0;
    for i in (k + 1)..=n {
        if i - (k + 1) > m {
            break;
        }
        div += (((factorial[i - 1] * modinv(factorial[i - k - 1]) % modulo) * modinv(factorial[k])
            % modulo)
            * ((factorial[n + m - (2 * i - (k + 1))] * modinv(factorial[n - i]) % modulo)
                * modinv(factorial[n + m - (2 * i - (k + 1)) - (n - i)])
                % modulo)
            % modulo) as i64;
        div %= modulo as i64;
    }

    sum -= div;
    while sum < 0 {
        sum += modulo as i64
    }

    println!("{}", sum);
}
