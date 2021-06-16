use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut deque = VecDeque::new();
    for i in 0..n {
        if i % 2 == 0 {
            deque.push_back(a[i]);
        } else {
            deque.push_front(a[i])
        }
    }

    for i in 0..n {
        if n % 2 == 0 {
            println!("{}", deque[i]);
        } else {
            println!("{}", deque[n - 1 - i]);
        }
    }
}
