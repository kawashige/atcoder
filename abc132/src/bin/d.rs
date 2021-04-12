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

    let mut factorials = vec![1; n + 1];
    for i in 2..=n {
        factorials[i] = (factorials[i - 1] * i) % m;
    }

    for i in 1..=k {
        if i - 1 > n - k {
            println!("0");
        } else {
            let result =
                (factorials[k - 1] * modinv(factorials[i - 1]) % m) * modinv(factorials[k - i]) % m;
            let result2 = (factorials[n - k + 1] * modinv(factorials[n - k + 1 - i]) % m)
                * modinv(factorials[i])
                % m;

            println!("{}", result * result2 % m);
        }
    }
}
