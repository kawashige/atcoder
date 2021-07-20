use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n]
    }

    a.sort_unstable();
    let mut deque1 = VecDeque::from(a.clone());
    let mut deque2 = VecDeque::new();
    deque2.push_back(deque1.pop_front().unwrap());

    let mut sum: i64 = 0;

    while !deque1.is_empty() {
        for _ in 0..2 {
            if let Some(x) = deque1.pop_back() {
                if deque2[0] < deque2[deque2.len() - 1] {
                    sum += (x - deque2[0]).abs() as i64;
                    deque2.push_front(x);
                } else {
                    sum += (x - deque2[deque2.len() - 1]).abs() as i64;
                    deque2.push_back(x);
                }
            }
        }
        for _ in 0..2 {
            if let Some(x) = deque1.pop_front() {
                if deque2[0] > deque2[deque2.len() - 1] {
                    sum += (x - deque2[0]).abs() as i64;
                    deque2.push_front(x);
                } else {
                    sum += (x - deque2[deque2.len() - 1]).abs() as i64;
                    deque2.push_back(x);
                }
            }
        }
    }

    a.sort_unstable_by(|a, b| b.cmp(&a));
    let mut deque1 = VecDeque::from(a.clone());
    let mut deque2 = VecDeque::new();
    deque2.push_back(deque1.pop_front().unwrap());

    let mut sum2: i64 = 0;

    while !deque1.is_empty() {
        for _ in 0..2 {
            if let Some(x) = deque1.pop_back() {
                if deque2[0] > deque2[deque2.len() - 1] {
                    sum2 += (x - deque2[0]).abs() as i64;
                    deque2.push_front(x);
                } else {
                    sum2 += (x - deque2[deque2.len() - 1]).abs() as i64;
                    deque2.push_back(x);
                }
            }
        }
        for _ in 0..2 {
            if let Some(x) = deque1.pop_front() {
                if deque2[0] < deque2[deque2.len() - 1] {
                    sum2 += (x - deque2[0]).abs() as i64;
                    deque2.push_front(x);
                } else {
                    sum2 += (x - deque2[deque2.len() - 1]).abs() as i64;
                    deque2.push_back(x);
                }
            }
        }
    }

    println!("{}", sum.max(sum2));
}
