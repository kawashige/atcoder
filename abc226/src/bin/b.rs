use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut set = HashSet::new();
    for _ in 0..n {
        input! {
            l: usize,
            a: [String; l]
        }

        set.insert(a.join(","));
    }

    println!("{}", set.len());
}
