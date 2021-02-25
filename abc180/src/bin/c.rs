use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: u64
    }

    let max = (n as f64).sqrt() as u64;
    let mut set = BTreeSet::new();

    for i in 1..=max {
        if n % i == 0 {
            set.insert(i);
            set.insert(n / i);
        }
    }

    for i in set {
        println!("{}", i);
    }
}
