use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize
    }

    println!("{}", (n - 1) * (m - 1));
}
