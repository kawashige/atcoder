use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        results: [(usize, String); m]
    }

    let mut map = HashMap::new();
    for (p, s) in results {
        if s == "AC" {
            map.entry(p).or_insert((0, 0)).0 += 1;
        } else {
            let entry = map.entry(p).or_insert((0, 0));
            if entry.0 == 0 {
                entry.1 += 1;
            }
        }
    }

    let mut a = 0;
    let mut p = 0;

    for (ac, wa) in map.values() {
        if ac > &0 {
            a += 1;
            p += wa;
        }
    }

    println!("{} {}", a, p);
}
