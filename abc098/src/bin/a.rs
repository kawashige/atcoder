use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }

    let r = (a + b).max(a - b).max(a * b);

    println!("{}", r);
}
