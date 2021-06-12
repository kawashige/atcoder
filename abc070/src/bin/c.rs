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
        n: usize,
        t: [u64; n]
    }

    let mut x = t[0];
    for i in 1..n {
        x = lcm(x, t[i]);
    }

    println!("{}", x);
}
