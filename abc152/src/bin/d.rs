use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize
    }

    let mut map = HashMap::new();
    let mut count = 0;
    for i in 1..=n {
        let chars = i.to_string().chars().collect::<Vec<char>>();
        if chars.last() == chars.first() {
            count += 1;
        }
        if let Some(c) = map.get(&(*chars.last().unwrap(), *chars.first().unwrap())) {
            count += c * 2;
        }
        *map.entry((*chars.first().unwrap(), *chars.last().unwrap()))
            .or_insert(0) += 1;
    }

    println!("{}", count);
}
