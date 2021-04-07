use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }

    let mut map = HashMap::new();
    let mut count: usize = 0;

    for mut ss in s {
        ss.sort_unstable();
        let str = ss.into_iter().collect::<String>();
        if let Some(c) = map.get(&str) {
            count += c;
        }
        *map.entry(str).or_insert(0) += 1;
    }

    println!("{}", count);
}
