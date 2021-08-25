use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let map = s.into_iter().fold(HashMap::new(), |mut map, name| {
        *map.entry(name).or_insert(1) += 1;
        map
    });

    let mut v = map.into_iter().collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    println!("{}", v[0].0);
}
