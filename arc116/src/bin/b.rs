use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    a.sort_unstable();

    const M: usize = 998244353;

    let mut mul = 0;
    let mut r = 0;

    for i in (0..n).rev() {
        r += a[i] * a[i] % M;
        r %= M;
        r += mul * a[i] % M;
        r %= M;

        mul = mul * 2 % M + a[i];
        mul %= M;
    }

    println!("{}", r);
}
