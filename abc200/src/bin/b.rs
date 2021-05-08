use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut n = n as u128;
    for _ in 0..k {
        if n % 200 == 0 {
            n /= 200;
        } else {
            n = n * 1000 + 200;
        }
    }
    println!("{}", n);
}
