use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize
    }

    println!("{}", (x - 1).min(n - x));
}
