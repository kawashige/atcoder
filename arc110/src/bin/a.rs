use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}
fn main() {
    input! {
        n: u64
    }

    println!("{}", (2..=n).fold(1, |acc, x| lcm(acc, x)) + 1);
}
