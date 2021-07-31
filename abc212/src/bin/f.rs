use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        mut abst: [(usize, usize, usize, usize); m],
        xyz: [(usize, usize, usize); q]
    }

    let mut passenger = vec![BinaryHeap::new(); n];
    let mut result = vec![vec![]; q];

    for (i, (x, y, z)) in xyz.into_iter().enumerate() {
        passenger[y - 1].push((Reverse(x), z, i));
        result[i] = vec![y];
    }

    abst.sort_unstable_by(|a, b| a.2.cmp(&b.2));

    for (a, b, s, t) in abst {
        while let Some((Reverse(time), z, i)) = passenger[a - 1].pop() {
            if time > s {
                passenger[a - 1].push((Reverse(time), z, i));
                break;
            }

            if z <= s {
                result[i] = vec![a];
            } else if z <= t {
                result[i] = vec![a, b];
            } else {
                passenger[b - 1].push((Reverse(t), z, i));
                result[i] = vec![b];
            }
        }
    }

    for v in result {
        println!(
            "{}",
            v.into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
