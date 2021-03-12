use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    println!("{}", s.into_iter().collect::<HashSet<String>>().len());
}
