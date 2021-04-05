use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    let mut count = 0;
    while 1 + (a - 1) * count < b {
        count += 1;
    }

    println!("{}", count);
}
