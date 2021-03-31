use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }
    if 1 <= a && a <= 9 && 1 <= b && b <= 9 {
        println!("{}", a * b);
    } else {
        println!("-1");
    }
}
