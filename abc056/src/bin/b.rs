use proconio::input;

fn main() {
    input! {
        w: usize,
        a: usize,
        b: usize,
    }

    if a + w < b {
        println!("{}", b - (a + w));
    } else if b + w < a {
        println!("{}", a - (b + w));
    } else {
        println!("0");
    }
}
