use proconio::input;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut new_s = Vec::with_capacity(n);
    let mut map = HashMap::new();
    let mut map2 = HashMap::new();
    for i in 0..n {
        (*map.entry(s[i][..3].to_string()).or_insert(Vec::new())).push(i);
        (*map2
            .entry(s[i][(s[i].len() - 3)..].to_string())
            .or_insert(Vec::new()))
        .push(i);
        new_s.push((s[i][..3].to_string(), s[i][(s[i].len() - 3)..].to_string()));
    }

    let mut count = vec![0; n];
    let mut queue = VecDeque::new();
    let mut win = vec![2; n];
    for i in 0..n {
        count[i] = map.get(&new_s[i].1).unwrap_or(&vec![]).len();
        if count[i] == 0 {
            queue.push_back(i);
            win[i] = 1;
        }
    }

    while let Some(i) = queue.pop_front() {
        if let Some(v) = map2.get(&new_s[i].0) {
            for x in v {
                if &i == x {
                    continue;
                }
                count[*x] -= 1;
                if count[*x] == 0 {
                    queue.push_back(*x);
                    if let Some(vv) = map.get(&new_s[*x].1) {
                        if vv.iter().all(|i| win[*i] == 0) {
                            win[*x] = 1;
                        } else if vv.iter().any(|i| win[*i] == 1) {
                            win[*x] = 0;
                        }
                    }
                }
            }
        }
    }

    for i in 0..n {
        if win[i] == 1 {
            println!("Takahashi");
        } else if win[i] == 0 {
            println!("Aoki");
        } else {
            println!("Draw");
        }
    }
}
