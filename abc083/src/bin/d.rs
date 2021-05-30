use std::collections::HashSet;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    if s.iter().collect::<HashSet<&char>>().len() == 1 {
        println!("{}", s.len());
        return;
    }

    let mut max_zero = std::usize::MAX;
    for i in 0..s.len() {
        if s[i] == '0' {
            max_zero = std::cmp::min(max_zero, std::cmp::max(i, s.len() - 1 - i));
        }
    }

    let mut max_one = std::usize::MAX;
    for i in 0..s.len() {
        if s[i] == '1' {
            max_one = std::cmp::min(max_one, std::cmp::max(i, s.len() - 1 - i));
        }
    }

    println!("{}", std::cmp::max(max_zero, max_one));
}
