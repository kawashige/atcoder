use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [[usize]; n]
    }

    let mut map = HashMap::new();
    map.insert(x, 1);

    for i in 0..n {
        let mut new_map = HashMap::new();
        for j in 0..a[i].len() {
            for (k, v) in &map {
                if k % a[i][j] == 0 {
                    *new_map.entry(k / a[i][j]).or_insert(0) += v;
                }
            }
        }
        map = new_map;
    }

    println!("{}", map.get(&1).unwrap_or(&0));
}
