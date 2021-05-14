use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        _a: [usize; n]
    }

    let count = (n - 1 + k - 2) / (k - 1);

    println!("{}", count)
}
