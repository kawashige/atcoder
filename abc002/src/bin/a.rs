use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize
    }

    println!("{}", x.max(y));
}
