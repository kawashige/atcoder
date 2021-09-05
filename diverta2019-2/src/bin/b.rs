use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n]
    }
    xy.sort_unstable();

    let mut map = HashMap::new();
    for i in 0..n {
        for j in (i + 1)..n {
            *map.entry((xy[j].0 - xy[i].0, xy[j].1 - xy[i].1))
                .or_insert(0) += 1;
        }
    }

    let count = map.values().max().unwrap_or(&0);

    println!("{}", n - count);
}
