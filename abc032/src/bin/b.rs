use std::collections::HashSet;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        k: usize
    }

    let set = s.windows(k).collect::<HashSet<_>>();

    println!("{}", set.len());
}
