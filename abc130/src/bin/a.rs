use proconio::input;

fn main() {
    input! {
        x: usize,
        a: usize
    }

    if x < a {
        println!("0");
    } else {
        println!("10");
    }
}
