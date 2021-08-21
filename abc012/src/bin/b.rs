use proconio::input;

fn main() {
    input! {
        n: usize
    }

    println!(
        "{:>02}:{:>02}:{:>02}",
        n / 3600,
        (n % 3600) / 60,
        (n % 3600) % 60
    );
}
