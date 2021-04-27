use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }

    if b % a == 0 {
        println!("{}", a + b);
    } else {
        println!("{}", b - a);
    }
}
