use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [[String; 2]; n]
    }

    let set = st
        .into_iter()
        .map(|v| v.join(" "))
        .collect::<HashSet<String>>();

    if set.len() == n {
        println!("No");
    } else {
        println!("Yes");
    }
}
