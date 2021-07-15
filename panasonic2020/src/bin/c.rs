use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }

    if c > a + b && 4 * a * b < (c - a - b) * (c - a - b) {
        println!("Yes");
    } else {
        println!("No");
    }
}
