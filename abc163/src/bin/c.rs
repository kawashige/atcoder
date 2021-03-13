use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1]
    }

    let mut map = HashMap::new();
    for i in a {
        *map.entry(i).or_insert(0) += 1;
    }
    for i in 1..=n {
        let count = map.get(&i).unwrap_or(&0);
        println!("{}", count);
    }
}
