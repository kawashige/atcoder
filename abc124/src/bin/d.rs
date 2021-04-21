use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    }

    let mut blacks = VecDeque::new();
    blacks.push_back((0, 0));
    let mut max = 0;
    for i in 1..n {
        match (s[i - 1], s[i]) {
            ('0', '1') => {
                if blacks.len() == k + 1 {
                    max = std::cmp::max(max, blacks[blacks.len() - 1].1 - blacks[0].0 + 1);
                    blacks.pop_front();
                }
                blacks.push_back((i, i));
            }
            ('1', '1') => {
                let l = blacks.len();
                blacks[l - 1].1 = i;
            }
            _ => {}
        }
    }
    max = std::cmp::max(max, blacks[blacks.len() - 1].1 - blacks[0].0 + 1);
    if s[n - 1] == '0' {
        if blacks.len() == k + 1 {
            blacks.pop_front();
        }
        max = std::cmp::max(max, n - blacks[0].0);
    }

    println!("{}", max);
}
