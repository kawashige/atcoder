use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    if a.into_iter().collect::<HashSet<usize>>().len() == n {
        println!("YES");
    } else {
        println!("NO");
    }
}
