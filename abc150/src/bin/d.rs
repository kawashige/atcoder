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
        m: u64,
        a: [u64; n]
    }

    let mut l = a[0] / 2;
    for i in 1..n {
        l = lcm(l, a[i] / 2);
    }
    for i in 0..n {
        if (l / (a[i] / 2)) % 2 == 0 {
            println!("0");
            return;
        }
    }
    let count = if l > m { 0 } else { 1 + (m - l) / (2 * l) };
    println!("{}", count);
}
