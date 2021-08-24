use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let r = a
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .rev()
        .skip(1)
        .next()
        .unwrap();
    println!("{}", r);
}
