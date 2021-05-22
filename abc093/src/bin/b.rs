use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        k: i32
    }

    let mut set = BTreeSet::new();
    for i in a..std::cmp::min(a + k, b + 1) {
        set.insert(i);
    }
    for i in std::cmp::max(a, b + 1 - k)..=b {
        set.insert(i);
    }

    for i in set {
        println!("{}", i);
    }
}
