use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [i32; n],
        y: [i32; m],
    }

    const M: usize = 1_000_000_007;

    let mut r = 0;

    let mut y_sum = 0;
    for i in 1..m {
        let count = i * (m - i) % M;
        y_sum += count * (y[i] - y[i - 1]) as usize % M;
        y_sum %= M;
    }

    for i in 1..n {
        let count = i * (n - i) % M;
        r += (count * (x[i] - x[i - 1]) as usize % M) * y_sum % M;
        r %= M;
    }

    println!("{}", r);
}
