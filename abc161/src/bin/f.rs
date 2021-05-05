use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: u64
    }

    let max = (n as f64).sqrt() as u64;
    let mut factors = HashSet::new();

    for i in 2..=max {
        let mut x = n;
        while x % i == 0 {
            x /= i;
        }
        if x == 1 || (x - 1) % i == 0 {
            factors.insert(i);
        }
    }

    let max = ((n - 1) as f64).sqrt() as u64;
    for i in 1..=max {
        if (n - 1) % i == 0 {
            factors.insert(i);
            factors.insert((n - 1) / i);
        }
    }

    println!("{}", factors.len());
}
