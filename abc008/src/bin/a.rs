use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize
    }

    println!("{}", t - s + 1);
}
