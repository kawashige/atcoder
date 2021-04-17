use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m]
    }

    let mut map = BTreeMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }
    for i in 0..m {
        *map.entry(b[i]).or_insert(0) += 1;
    }

    let r = map
        .into_iter()
        .filter(|(_, c)| c == &1)
        .map(|(k, _)| k.to_string())
        .collect::<Vec<String>>();
    println!("{}", r.join(" "));
}
