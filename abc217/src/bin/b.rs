use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        s: [String; 3]
    }

    let mut set = vec![
        "ABC".to_string(),
        "ARC".to_string(),
        "AGC".to_string(),
        "AHC".to_string(),
    ]
    .into_iter()
    .collect::<HashSet<_>>();

    for x in s {
        set.remove(&x);
    }

    println!("{}", set.into_iter().next().unwrap());
}
