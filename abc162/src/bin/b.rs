use proconio::input;

fn main() {
    input! {
        n: u128
    }

    let sum = (1..=n).filter(|i| i % 3 != 0 && i % 5 != 0).sum::<u128>();
    println!("{}", sum);
}
