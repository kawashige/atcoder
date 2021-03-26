use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn main() {
    input! {
        a: usize,
        b: usize
    }

    println!("{}", lcm(a, b));
}
