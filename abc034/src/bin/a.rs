use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize
    }

    if x < y {
        println!("Better");
    } else {
        println!("Worse");
    }
}
