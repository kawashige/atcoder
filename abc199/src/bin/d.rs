use alloc::collections;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut lists = vec![vec![]; n];
    for (a, b) in ab {
        lists[a - 1].push(b - 1);
        lists[b - 1].push(a - 1);
    }

    if lists.is_empty() {
        println!("{}", 3_u64.pow(n as u32));
        return;
    }

    if lists.iter().any(|v| v.len() >= 4) {
        println!("0");
        return;
    }
}
