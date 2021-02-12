use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize
    }

    let count = b / x + 1 - (if a == 0 { 0 } else { (a - 1) / x + 1 });
    println!("{}", count);
}
