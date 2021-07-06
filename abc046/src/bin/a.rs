use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        v: [usize; 3],
    }

    let c = v.into_iter().collect::<HashSet<usize>>().len();

    println!("{}", c);
}
