use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize
    }
    if a + b == c + d {
        println!("Balanced");
    } else if a + b < c + d {
        println!("Right");
    } else {
        println!("Left");
    }
}
