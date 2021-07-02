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

fn count(x: u64) -> u64 {
    const M: u64 = 1_000_000_007;

    let l = x.to_string().len() as u64;
    let mut count = 0;

    for i in 1..l {
        count += (((i * 9 % M) * 10_u64.pow(i as u32 - 1) % M)
            * ((10_u64.pow(i as u32) + 10_u64.pow(i as u32 - 1) - 1) % M))
            % M
            * modinv(2)
            % M;
        count %= M;
    }

    count +=
        ((l * (x - (10_u64.pow(l as u32 - 1) - 1)) % M) * ((10_u64.pow(l as u32 - 1) + x) % M) % M)
            * modinv(2)
            % M;
    count % M
}

fn main() {
    input! {
        l: u64,
        r: u64
    }

    const M: u64 = 1_000_000_007;
    let mut c = count(r) as i64;
    if l > 1 {
        c -= count(l - 1) as i64;
        if c < 0 {
            c += M as i64;
        }
    }

    println!("{}", c);
}
