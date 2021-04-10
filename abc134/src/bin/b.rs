use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize
    }

    println!("{}", (n + 2 * d) / (2 * d + 1));
}
