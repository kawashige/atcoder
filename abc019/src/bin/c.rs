use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut set = HashSet::new();

    for x in a {
        let mut x = x;
        while x % 2 == 0 {
            x /= 2;
        }
        set.insert(x);
    }

    println!("{}", set.len());
}
