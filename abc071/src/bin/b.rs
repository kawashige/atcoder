use std::collections::HashSet;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let set = s.into_iter().collect::<HashSet<char>>();

    for i in 0..26 {
        let c = ('a' as u8 + i) as char;
        if !set.contains(&c) {
            println!("{}", c);
            return;
        }
    }
    println!("None");
}
