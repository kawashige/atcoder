use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i128; n]
    }

    let mut map = HashMap::new();
    let mut count: i128 = 0;
    for i in (0..n).rev() {
        if let Some(c) = map.get(&(a[i] + i as i128)) {
            count += c;
        }
        *map.entry(i as i128 - a[i]).or_insert(0) += 1;
    }
    println!("{}", count);
}
