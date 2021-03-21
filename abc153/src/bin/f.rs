use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: u64,
        a: u64,
        mut xh: [(u64, u64); n]
    }

    xh.sort_unstable_by_key(|x| x.0);

    let mut deque: VecDeque<(u64, u64)> = VecDeque::new();
    let mut sum = 0;
    let mut count = 0;
    for i in 0..n {
        while !deque.is_empty() && deque[0].0 < xh[i].0 {
            sum -= deque.pop_front().unwrap().1;
        }
        if xh[i].1 <= sum {
            continue;
        }
        let remains = xh[i].1 - sum;
        let c = (remains + a - 1) / a;
        count += c;
        deque.push_back((xh[i].0 + 2 * d, c * a));
        sum += c * a;
    }

    println!("{}", count);
}
