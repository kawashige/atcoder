use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }

    if a < b {
        println!("{}", (b - a).min(a + 10 - b));
    } else {
        println!("{}", (a - b).min(b + 10 - a));
    }
}
