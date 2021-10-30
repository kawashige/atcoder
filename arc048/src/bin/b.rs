use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        rh: [(usize, usize); n]
    }

    let mut count = BTreeMap::new();
    for (r, h) in &rh {
        let entry = count.entry(*r).or_insert([0; 4]);
        entry[h - 1] += 1;
    }

    let mut c = 0;
    let keys = count.keys().rev().cloned().collect::<Vec<_>>();
    for key in keys {
        let entry = count.get_mut(&key).unwrap();
        entry[3] = c;
        c += entry[..3].iter().sum::<usize>();
    }

    for (r, h) in rh {
        let c = count[&r];
        let w = n - c[0] - c[1] - c[2] - c[3] + c[h % 3];
        let l = c[3] + c[(h + 1) % 3];
        let d = c[h - 1] - 1;

        println!("{} {} {}", w, l, d)
    }
}
