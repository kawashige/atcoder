use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        n: usize,
        rc: [(usize, usize); n]
    }

    let mut r_count = vec![0; r];
    let mut c_count = vec![0; c];

    for (row, column) in &rc {
        r_count[row - 1] += 1;
        c_count[column - 1] += 1;
    }

    let mut r_map = HashMap::new();
    for i in 0..r_count.len() {
        *r_map.entry(r_count[i]).or_insert(0) += 1;
    }

    let mut r: usize = 0;
    for i in 0..c_count.len() {
        if k < c_count[i] {
            continue;
        }
        if let Some(x) = r_map.get(&(k - c_count[i])) {
            r += *x;
        }
    }

    for (row, column) in &rc {
        if c_count[column - 1] + r_count[row - 1] == k {
            r -= 1;
        }
        if c_count[*column - 1] + r_count[row - 1] - 1 == k {
            r += 1;
        }
    }

    println!("{}", r);
}
