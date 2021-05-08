use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let map = a.into_iter().fold(HashMap::new(), |mut map, x| {
        *map.entry(x % 200).or_insert(0) += 1;
        map
    });

    let mut count: u64 = 0;
    for (_, v) in map {
        count += (v * (v - 1)) / 2;
    }

    println!("{}", count);
}
