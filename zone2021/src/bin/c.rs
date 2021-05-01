use std::collections::{BinaryHeap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [[usize; 5]; n]
    }

    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);
    let mut c = Vec::with_capacity(n);
    let mut d = Vec::with_capacity(n);
    let mut e = Vec::with_capacity(n);

    for i in 0..n {
        a.push((v[i][0], i));
        b.push((v[i][1], i));
        c.push((v[i][2], i));
        d.push((v[i][3], i));
        e.push((v[i][4], i));
    }
    a.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    b.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    c.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    d.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    e.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    let mut heap = BinaryHeap::new();
    heap.push((
        *vec![a[0].0, b[0].0, c[0].0, d[0].0, e[0].0]
            .iter()
            .min()
            .unwrap(),
        vec![0, 0, 0, 0, 0],
    ));

    while let Some((max, v)) = heap.pop() {
        if vec![a[v[0]].1, b[v[1]].1, c[v[2]].1, d[v[3]].1, e[v[4]].1]
            .iter()
            .collect::<HashSet<&usize>>()
            .len()
            <= 3
        {
            println!("{}", max);
            return;
        }

        if v[0] + 1 < a.len() {
            heap.push((
                *vec![a[v[0] + 1].0, b[v[1]].0, c[v[2]].0, d[v[3]].0, e[v[4]].0]
                    .iter()
                    .min()
                    .unwrap(),
                vec![v[0] + 1, v[1], v[2], v[3], v[4]],
            ));
        }
        if v[1] + 1 < b.len() {
            heap.push((
                *vec![a[v[0]].0, b[v[1] + 1].0, c[v[2]].0, d[v[3]].0, e[v[4]].0]
                    .iter()
                    .min()
                    .unwrap(),
                vec![v[0], v[1] + 1, v[2], v[3], v[4]],
            ));
        }
        if v[2] + 1 < c.len() {
            heap.push((
                *vec![a[v[0]].0, b[v[1]].0, c[v[2] + 1].0, d[v[3]].0, e[v[4]].0]
                    .iter()
                    .min()
                    .unwrap(),
                vec![v[0], v[1], v[2] + 1, v[3], v[4]],
            ));
        }
        if v[3] + 1 < d.len() {
            heap.push((
                *vec![a[v[0]].0, b[v[1]].0, c[v[2]].0, d[v[3] + 1].0, e[v[4]].0]
                    .iter()
                    .min()
                    .unwrap(),
                vec![v[0], v[1], v[2], v[3] + 1, v[4]],
            ));
        }
        if v[4] + 1 < e.len() {
            heap.push((
                *vec![a[v[0]].0, b[v[1]].0, c[v[2]].0, d[v[3]].0, e[v[4] + 1].0]
                    .iter()
                    .min()
                    .unwrap(),
                vec![v[0], v[1], v[2], v[3], v[4] + 1],
            ));
        }
    }
}
