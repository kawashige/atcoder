use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut count = s
        .into_iter()
        .fold(HashMap::new(), |mut map, name| {
            *map.entry(name).or_insert(0) += 1;
            map
        })
        .into_iter()
        .collect::<Vec<_>>();

    count.sort_unstable_by_key(|(_, c)| *c);

    println!("{}", count.last().unwrap().0);
}
