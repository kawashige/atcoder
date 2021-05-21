use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars
    }

    if s[0] == '1' || s[n] == '1' {
        println!("-1");
        return;
    }

    let mut from = vec![-1; n + 1];
    let mut deque = VecDeque::new();
    deque.push_back(0);

    for i in 1..=n {
        if s[i] == '1' {
            continue;
        }
        while !deque.is_empty() && deque[0] + m < i {
            deque.pop_front();
        }
        if !deque.is_empty() {
            from[i] = deque[0] as i32;
        }
        deque.push_back(i);
    }

    let mut r = Vec::new();
    let mut i = n;
    while i > 0 {
        if from[i] == -1 {
            println!("-1");
            return;
        }
        r.push(i);
        i = from[i] as usize;
    }
    r.push(0);

    for i in (1..r.len()).rev() {
        println!("{:?}", r[i - 1] - r[i]);
    }
}
