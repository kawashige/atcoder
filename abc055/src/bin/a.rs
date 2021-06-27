use proconio::input;

fn main() {
    input! {
        n: usize
    }

    println!("{}", 800 * n - (n / 15) * 200);
}
