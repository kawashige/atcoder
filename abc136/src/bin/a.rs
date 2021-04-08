use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    if c <= a - b {
        println!("0");
    } else {
        println!("{}", c + b - a);
    }
}
