use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [char; n]
    }

    if s.into_iter().collect::<HashSet<char>>().len() == 3 {
        println!("Three");
    } else {
        println!("Four");
    }
}
