use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }

    let indices1 = ab
        .iter()
        .map(|(a, b)| vec![*a, a + b])
        .flatten()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .collect::<HashMap<_, _>>();
    let indices2 = indices1
        .clone()
        .into_iter()
        .map(|(i, x)| (x, i))
        .collect::<HashMap<_, _>>();

    let mut count: Vec<i32> = vec![0; indices1.len() + 1];

    for (a, b) in ab {
        count[indices2[&a]] += 1;
        count[indices2[&(a + b)]] -= 1;
    }

    let mut r = vec![0; n];
    let mut c = 0_i32;
    let mut c_i = 0;
    for i in 0..count.len() {
        if count[i] == 0 {
            continue;
        }
        if c > 0 {
            r[c as usize - 1] += indices1[&i] - indices1[&c_i];
        }
        c += count[i];
        c_i = i;
    }

    println!(
        "{}",
        r.into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
