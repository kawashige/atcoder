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
        x: usize,
        y: usize
    }

    let mut factorial = vec![0; std::cmp::max(2, std::cmp::max(x, y)) as usize];
    factorial[1] = 1;
    for i in 2..factorial.len() {
        factorial[i] = factorial[i - 1] * i % 1_000_000_007;
    }

    let mut sum = 0;

    for i in (if x % 2 == 0 { 0 } else { 1 }..=x).step_by(2) {
        let j = (x - i) / 2;

        if i * 2 + j != y {
            continue;
        }

        sum = if i == 0 || j == 0 {
            sum + 1
        } else {
            (sum + factorial[(i + j) as usize] * modinv(factorial[i as usize]) % 1_000_000_007
                * modinv(factorial[j as usize])
                % 1_000_000_007
                % 1_000_000_007)
                % 1_000_000_007
        };
    }

    println!("{}", sum);
}
