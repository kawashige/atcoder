use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n]
    }

    let mut set = HashSet::new();
    for i in 1..143 {
        for j in 1..143 {
            set.insert(4 * i * j + 3 * i + 3 * j);
        }
    }

    let c = s.into_iter().filter(|x| !set.contains(&x)).count();
    println!("{}", c);
}
