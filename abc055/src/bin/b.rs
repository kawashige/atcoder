use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let p = (1..=n).fold(1, |acc, n| (acc * n) % 1_000_000_007);
    println!("{}", p);
}
