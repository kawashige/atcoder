use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n]
    }

    let mut count: i64 = 0;
    let mut sum: i64 = 0;
    let mut map = HashMap::new();
    map.insert(0, 1);

    for i in 0..n {
        sum += a[i];
        count += map.get(&(sum - k)).unwrap_or(&0);
        *map.entry(sum).or_insert(0) += 1;
    }

    println!("{}", count);
}
