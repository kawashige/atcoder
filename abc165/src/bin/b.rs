use proconio::input;

fn main() {
    input! {
        x: u128
    }

    let mut n = 100;
    let mut i = 0;
    while x > n {
        n += n / 100;
        i += 1;
    }
    println!("{}", i);
}
