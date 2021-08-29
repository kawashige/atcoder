use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize]; m]

    }

    let mut list = vec![vec![]; n];
    let mut count = vec![0; n];

    for i in 0..m {
        for j in 1..a[i].len() {
            if list[a[i][j - 1] - 1].is_empty() || list[a[i][j - 1] - 1][0] != a[i][j] - 1 {
                list[a[i][j - 1] - 1].push(a[i][j] - 1);
                count[a[i][j] - 1] += 1;
            }
        }
    }

    let mut deque = VecDeque::new();

    for i in 0..n {
        if count[i] == 0 {
            deque.push_back(i);
        }
    }

    while let Some(v) = deque.pop_front() {
        for j in &list[v] {
            count[*j] -= 1;
            if count[*j] == 0 {
                deque.push_back(*j);
            }
        }
    }

    if count.into_iter().all(|c| c == 0) {
        println!("Yes");
    } else {
        println!("No");
    }
}
