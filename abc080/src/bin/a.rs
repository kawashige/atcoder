use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    }

    println!("{}", std::cmp::min(n * a, b));
}
