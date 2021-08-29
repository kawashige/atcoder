use proconio::input;

fn main() {
    input! {
        n: usize
    }

    println!("{}", (n * (n + 1) / 2 * 10000) as f64 / n as f64);
}
