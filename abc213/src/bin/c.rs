use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        mut ab: [(usize, usize); n]
    }

    let org = ab.clone();

    let mut h_map = HashMap::new();
    let mut w_map = HashMap::new();

    ab.sort_unstable();
    let mut x = 1;
    for i in 0..n {
        if i > 0 && ab[i - 1].0 == ab[i].0 {
            continue;
        }

        h_map.insert(ab[i].0, x);
        x += 1;
    }

    ab.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    let mut x = 1;
    for i in 0..n {
        if i > 0 && ab[i - 1].1 == ab[i].1 {
            continue;
        }

        w_map.insert(ab[i].1, x);
        x += 1;
    }

    for (a, b) in org {
        println!("{} {}", h_map.get(&a).unwrap(), w_map.get(&b).unwrap());
    }
}
