use proconio::input;

fn main() {
    input! {
        n: u128,
        a: u128,
        b: u128
    }

    println!("{}", a * (n / (a + b)) + std::cmp::min(n % (a + b), a));
}
