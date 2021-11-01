use std::collections::HashSet;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let c = s.into_iter().collect::<HashSet<_>>().len();
    let r = if c == 1 {
        1
    } else if c == 2 {
        3
    } else {
        6
    };

    println!("{}", r);
}
