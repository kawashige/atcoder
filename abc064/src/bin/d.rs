use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars
    }

    let mut count = 0;
    let mut r = VecDeque::new();
    for c in s {
        match c {
            '(' => {
                count += 1;
            }
            ')' if count == 0 => {
                r.push_front('(');
            }
            ')' => {
                count -= 1;
            }
            _ => unreachable!(),
        }
        r.push_back(c);
    }
    for _ in 0..count {
        r.push_back(')');
    }

    println!("{}", r.into_iter().collect::<String>());
}
