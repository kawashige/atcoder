use std::collections::HashSet;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        w: [Chars; n]
    }

    if w.iter().collect::<HashSet<&Vec<char>>>().len() == n
        && (1..n).all(|i| w[i - 1].last().unwrap() == &w[i][0])
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
