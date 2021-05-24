use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
        m: usize,
        t: [String; m]
    }

    let mut point = HashMap::new();
    for x in s {
        *point.entry(x).or_insert(0) += 1;
    }
    for x in t {
        *point.entry(x).or_insert(0) -= 1;
    }

    let mut p = point.values().collect::<Vec<_>>();
    p.sort_unstable();
    println!("{}", p.last().unwrap().max(&&0));
}
