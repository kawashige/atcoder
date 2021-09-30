use proconio::input;

fn f(b: i64, x: i64) -> i64 {
    0.max(b - (x - (b + 1)).abs())
}

fn main() {
    input! {
        n: i64,
        d: i64
    }

    const M: i64 = 998244353;

    let mut pow = vec![1; 2_000_001];
    for i in 1..pow.len() {
        pow[i] = pow[i - 1] * 2 % M;
    }

    let mut r = 0;

    if d != 1 {
        for i in 0..n {
            r += (pow[i as usize] * pow[d as usize - 2] % M) * f(n - 1 - i, d) % M;
            r %= M;
        }
    }

    if n > d {
        for i in 0..(n - d) {
            r += pow[i as usize] * pow[d as usize] % M;
            r %= M;
        }
    }

    println!("{}", r * 2 % M);
}
