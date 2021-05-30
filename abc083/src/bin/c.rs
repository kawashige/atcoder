use proconio::input;

fn main() {
    input! {
        x: u64,
        y: u64
    }

    let mut count = 1;
    let mut num = x;
    while num * 2 <= y {
        count += 1;
        num *= 2;
    }

    println!("{}", count);
}
