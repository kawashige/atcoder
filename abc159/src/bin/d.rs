use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }

    let mut count = HashMap::new();
    for x in &a {
        *count.entry(x).or_insert(0) += 1;
    }
    let sum: u64 = count.values().map(|v| v * (v - 1) / 2).sum();
    for i in 0..n {
        let v = count[&a[i]];
        println!("{}", sum + 1 - v);
    }
}
