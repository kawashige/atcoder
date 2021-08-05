use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64
    }

    let p = (1 + 6 * (k - 1) * (n - k) + 3 * (k - 1) + 3 * (n - k)) as f64 / (n * n * n) as f64;
    println!("{}", p);
}
