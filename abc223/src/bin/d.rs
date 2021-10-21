use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut count = vec![0; n + 1];
    let mut succeed = vec![vec![]; n + 1];

    for (a, b) in ab {
        succeed[a].push(b);
        count[b] += 1;
    }

    let mut heap = BinaryHeap::new();
    let mut r = Vec::with_capacity(n);

    for i in 1..=n {
        if count[i] == 0 {
            heap.push(Reverse(i));
        }
    }

    while let Some(Reverse(i)) = heap.pop() {
        r.push(i);
        for j in &succeed[i] {
            count[*j] -= 1;
            if count[*j] == 0 {
                heap.push(Reverse(*j));
            }
        }
    }

    if r.len() == n {
        println!(
            "{}",
            r.into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    } else {
        println!("-1")
    }
}
