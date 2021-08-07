use std::collections::HashMap;

use proconio::input;

fn recurse(i: usize, k: usize, a: &Vec<i64>, memo: &mut HashMap<(usize, usize), i64>) -> i64 {
    if i >= a.len() {
        return std::i64::MIN;
    }
    if let Some(x) = memo.get(&(i, k)) {
        return *x;
    }

    let mut max = std::i64::MIN;
    for j in i..(a.len() - if k == 1 { 0 } else { 2 * (k - 1) - 1 }) {
        if k == 1 {
            max = max.max(a[j]);
        } else {
            let r = recurse(j + 2, k - 1, a, memo);
            if r != std::i64::MIN {
                max = max.max(a[j] + r);
            }
        }
    }

    memo.insert((i, k), max);
    max
}

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let r = recurse(0, a.len() / 2, &a, &mut HashMap::new());

    println!("{}", r);
}
