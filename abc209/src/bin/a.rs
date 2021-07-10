use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    if a > b {
        println!("0");
    } else {
        println!("{}", b - a + 1);
    }
}
