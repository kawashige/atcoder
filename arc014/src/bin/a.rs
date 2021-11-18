use proconio::input;

fn main() {
    input! {
        n: usize
    }

    println!("{}", if n % 2 == 1 { "Red" } else { "Blue" });
}
