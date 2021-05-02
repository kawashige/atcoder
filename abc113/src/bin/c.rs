use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        _n: usize,
        m: usize,
        py: [(usize, usize); m]
    }

    let map = py
        .into_iter()
        .zip(1..)
        .fold(HashMap::new(), |mut map, ((p, y), i)| {
            (*map.entry(p).or_insert(Vec::new())).push((i, y));
            map
        });

    let mut nums = map
        .into_iter()
        .flat_map(|(p, mut v)| {
            v.sort_unstable_by(|a, b| a.1.cmp(&b.1));
            v.into_iter()
                .zip(1..)
                .map(|((i, _), j)| (i, format!("{:06}{:06}", p, j)))
                .collect::<Vec<(i32, String)>>()
        })
        .collect::<Vec<(i32, String)>>();

    nums.sort_unstable();
    for (_, s) in nums {
        println!("{}", s);
    }
}
