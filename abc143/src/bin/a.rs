use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    if a <= b * 2 {
        println!("0")
    } else {
        println!("{}", a - b * 2);
    }
}
