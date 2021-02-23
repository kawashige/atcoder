use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        w: u128,
        peoples: [(usize, usize, u128); n],
    }

    let mut map = BTreeMap::new();
    for (s, t, p) in peoples {
        *map.entry(s).or_insert(0_i64) += p as i64;
        *map.entry(t).or_insert(0_i64) -= p as i64;
    }

    let mut uses = 0;
    for n in map.values() {
        uses += n;
        if w < uses as u128 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
