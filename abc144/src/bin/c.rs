use proconio::input;

fn main() {
    input! {
        n: u64
    }

    let mut min = std::u64::MAX;
    for i in 1..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            min = std::cmp::min(min, i + n / i);
        }
    }

    println!("{}", min - 2);
}
