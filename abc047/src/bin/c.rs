use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut deque = VecDeque::new();
    deque.push_back(1);
    for i in 1..s.len() {
        if s[i - 1] == s[i] {
            let l = deque.len();
            deque[l - 1] += 1;
        } else {
            deque.push_back(1);
        }
    }

    let mut c = 0;
    while deque.len() > 1 {
        let l = deque.len() - 1;
        if deque[0] > deque[l] {
            deque.pop_front();
        } else {
            deque.pop_back();
        }
        c += 1;
    }

    println!("{}", c);
}
