use std::collections::HashSet;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: Chars
    }

    let x = x
        .into_iter()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    if x.iter().collect::<HashSet<&usize>>().len() == 1
        || (0..3).all(|i| (x[i] + 1) % 10 == x[i + 1])
    {
        println!("Weak");
    } else {
        println!("Strong");
    }
}
