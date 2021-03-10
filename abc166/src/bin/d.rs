use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        x: i128,
    }

    let mut map = HashMap::new();

    let mut i: i128 = 0;
    loop {
        let p = i.pow(5);
        if let Some(j) = map.get(&p) {
            println!("{} {}", i, j);
            return;
        } else {
            map.insert(x + p, i);
            map.insert(x - p, -i);
        }
        i += 1;
    }
}
