use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    if a + b == 15 {
        println!("+")
    } else if a * b == 15 {
        println!("*")
    } else {
        println!("x")
    }
}
