use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        s: Chars
    }

    let mut t = VecDeque::new();
    let mut reversed = false;

    for c in s {
        if c == 'R' {
            reversed = !reversed;
        } else if reversed {
            t.push_front(c);
        } else {
            t.push_back(c);
        }
    }

    let v: Vec<char> = if reversed {
        t.into_iter().rev().collect()
    } else {
        t.into_iter().collect()
    };

    let mut stack = Vec::new();
    for c in v {
        if stack.last() == Some(&c) {
            stack.pop();
        } else {
            stack.push(c);
        }
    }

    println!("{}", stack.into_iter().collect::<String>());
}
