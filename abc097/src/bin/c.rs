use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        s: String,
        k: usize
    }

    let mut strings = (0..s.len())
        .map(|i| s[i..].to_string())
        .collect::<Vec<String>>();
    strings.sort_unstable_by(|a, b| b.cmp(&a));

    let mut k = k;
    let mut set = HashSet::new();
    while let Some(str) = strings.pop() {
        for i in 1..=str.len() {
            let s = (&str[..i]).to_string();
            if set.contains(&s) {
                continue;
            }
            k -= 1;
            if k == 0 {
                println!("{}", s);
                return;
            }
            set.insert(s);
        }
    }
}
