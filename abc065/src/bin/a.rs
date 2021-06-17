use proconio::input;

fn main() {
    input! {
        x: usize,
        a: usize,
        b: usize
    }

    if a >= b {
        println!("delicious");
    } else if a + x >= b {
        println!("safe");
    } else {
        println!("dangerous");
    }
}
