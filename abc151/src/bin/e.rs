use proconio::input;

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
        n: usize,
        k: usize,
        mut a: [i64; n]
    }

    a.sort_unstable();

    let mut sum = 0;

    let mut combination: i64 = 1;
    let mut mul: i64 = 1;
    sum += a[k - 1] - a[n - k];

    for i in k..n {
        let j = i - (k - 1);
        let l = n - k - j;
        combination =
            (combination * modinv(j as i64) % 1_000_000_007) * (k + j - 2) as i64 % 1_000_000_007;
        mul = (mul + combination) % 1_000_000_007;
        sum += (a[i] - a[l]) * mul % 1_000_000_007;
        sum %= 1_000_000_007;
    }

    println!("{}", sum)
}
