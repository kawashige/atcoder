use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut map = HashMap::new();
    map.insert(a[n - 1], 1);

    let mut r = 0;
    for i in (0..(n - 1)).rev() {
        r += n - 1 - i - map.get(&a[i]).unwrap_or(&0);
        *map.entry(a[i]).or_insert(0) += 1;
    }

    println!("{}", r);
}
