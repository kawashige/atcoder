use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n]
    }

    let mut count: u64 = 0;

    let mut remains_count = HashMap::new();
    let mut remains = 0;

    for x in a {
        let r = x % m;
        if r == 0 {
            count += 1;
        }
        let target = (remains + r) % m;
        if let Some(c) = remains_count.get(&target) {
            count += c;
        }

        *remains_count.entry(remains).or_insert(0) += 1;
        remains += r;
        remains %= m;
    }

    println!("{}", count);
}
