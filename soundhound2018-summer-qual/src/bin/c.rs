use proconio::input;

fn main() {
    input! {
        n: u64,
        m: u64,
        d: u64
    }

    let one = if d == 0 { n } else { (2 * d).min(n) };
    let two = n - one;

    let r = ((one + 2 * two) * (m - 1)) as f64 / (n * n) as f64;

    println!("{}", r);
}
