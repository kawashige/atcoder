use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut map = HashMap::new();
    map.insert(0, 1);
    let mut sum = 0;
    let mut r = 0_usize;

    for x in a {
        if let Some(i) = map.get(&(sum + x)) {
            r += i;
        }
        sum += x;
        *map.entry(sum).or_insert(0) += 1;
    }

    println!("{}", r);
}
