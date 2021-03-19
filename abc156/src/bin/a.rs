use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize
    }

    if n >= 10 {
        println!("{}", r);
    } else {
        println!("{}", r + 100 * (10 - n));
    }
}
