use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: String
    }

    if n.chars().collect::<HashSet<char>>().len() == 1 {
        println!("SAME");
    } else {
        println!("DIFFERENT")
    }
}
