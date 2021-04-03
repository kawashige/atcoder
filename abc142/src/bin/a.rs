use proconio::input;

fn main() {
    input! {
        n: usize
    }

    println!(
        "{}",
        (1..=n).filter(|i| i % 2 == 1).count() as f64 / n as f64
    )
}
