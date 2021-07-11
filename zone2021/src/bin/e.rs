use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        a: [[usize; c - 1]; r],
        b: [[usize; c]; r - 1],
    }

    let mut dist = vec![vec![std::usize::MAX; c]; r];
    dist[0][0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), (0, 0)));

    while let Some((Reverse(d), (i, j))) = heap.pop() {
        if dist[i][j] < d {
            continue;
        }

        if (i, j) == (r - 1, c - 1) {
            println!("{}", d);
            return;
        }

        if j < c - 1 {
            if dist[i][j + 1] > dist[i][j] + a[i][j] {
                dist[i][j + 1] = dist[i][j] + a[i][j];
                heap.push((Reverse(dist[i][j + 1]), (i, j + 1)));
            }
        }
        if j > 0 {
            if dist[i][j - 1] > dist[i][j] + a[i][j - 1] {
                dist[i][j - 1] = dist[i][j] + a[i][j - 1];
                heap.push((Reverse(dist[i][j - 1]), (i, j - 1)));
            }
        }
        if i < r - 1 {
            if dist[i + 1][j] > dist[i][j] + b[i][j] {
                dist[i + 1][j] = dist[i][j] + b[i][j];
                heap.push((Reverse(dist[i + 1][j]), (i + 1, j)));
            }
        }
        for k in 0..i {
            if dist[k][j] > dist[i][j] + i - k + 1 {
                dist[k][j] = dist[i][j] + i - k + 1;
                heap.push((Reverse(dist[k][j]), (k, j)));
            }
        }
    }
}
