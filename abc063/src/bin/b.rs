use std::collections::HashSet;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    if s.len() == s.iter().collect::<HashSet<&char>>().len() {
        println!("yes");
    } else {
        println!("no");
    }
}
