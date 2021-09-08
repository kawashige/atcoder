use proconio::input;

fn main() {
    input! {
        n: u64
    }

    println!("{}", (2..=n).product::<u64>() + 1);
}
