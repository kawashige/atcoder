use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    if a * b > c * d {
        println!("{}", a * b);
    } else {
        println!("{}", c * d);
    }
}
