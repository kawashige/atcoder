use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64
    }

    let x = gcd(gcd(a, b), c);
    println!("{}", a / x - 1 + b / x - 1 + c / x - 1);
}
