use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut counts = HashMap::new();
    for x in a {
        *counts.entry(x).or_insert(0) += 1
    }

    let mut count = 0;
    for (k, v) in counts {
        if k > v {
            count += v;
        } else {
            count += v - k;
        }
    }

    println!("{}", count);
}
