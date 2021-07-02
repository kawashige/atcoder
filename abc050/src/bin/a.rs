use proconio::input;

fn main() {
    input! {
        a: i32,
        op: char,
        b: i32
    }

    let r = if op == '+' { a + b } else { a - b };
    println!("{}", r);
}
