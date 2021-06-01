use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let mut map = HashMap::new();
    for x in a {
        *map.entry(x).or_insert(0) += 1;
    }
    let mut counts = map.into_iter().map(|(_, v)| v).collect::<Vec<usize>>();

    if counts.len() <= k {
        println!("0");
        return;
    }

    counts.sort_unstable();
    let l = counts.len();
    let sum = counts.into_iter().take(l - k).sum::<usize>();

    println!("{}", sum);
}
