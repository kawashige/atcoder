use proconio::input;

const M: usize = 1_000_000_007;

fn f(r: usize, c: usize, factorial: &Vec<usize>, mod_factorial: &Vec<usize>) -> usize {
    let mut ans = 0;
    for i in 1..(c + 2) {
        ans += (factorial[r + i] * mod_factorial[i] % M) * mod_factorial[r] % M;
        ans %= M;
    }
    ans
}

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
        r1: usize,
        c1: usize,
        r2: usize,
        c2: usize,
    }

    let mut factorial = vec![1; r2 + c2 + 3];
    let mut mod_factorial = vec![1; r2 + c2 + 3];
    for i in 2..factorial.len() {
        factorial[i] = (factorial[i - 1] * i) % M;
        mod_factorial[i] = modinv(factorial[i]);
    }

    let ans = ((f(r2, c2, &factorial, &mod_factorial)
        + (M - f(r2, c1 - 1, &factorial, &mod_factorial)) % M
        + (M - f(r1 - 1, c2, &factorial, &mod_factorial)) % M)
        + f(r1 - 1, c1 - 1, &factorial, &mod_factorial))
        % M;

    println!("{}", ans);
}
