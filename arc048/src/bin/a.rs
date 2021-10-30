use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }

    let mut r = b - a;
    if a < 0 && b > 0 {
        r -= 1;
    }

    println!("{}", r);
}
