use proconio::input;

fn main() {
    input! {
        x: u64
    }

    let r = if x % 11 == 0 {
        x / 11 * 2
    } else if x % 11 <= 6 {
        x / 11 * 2 + 1
    } else {
        (x + 10) / 11 * 2
    };

    println!("{}", r);
}
