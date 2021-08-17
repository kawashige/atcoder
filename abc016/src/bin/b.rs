use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32
    }

    if c == a + b && c == a - b {
        println!("?");
    } else if c == a + b {
        println!("+");
    } else if c == a - b {
        println!("-");
    } else {
        println!("!");
    }
}
