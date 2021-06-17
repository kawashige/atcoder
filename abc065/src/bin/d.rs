use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n]
    }

    let mut x = xy
        .iter()
        .enumerate()
        .map(|(i, (x, _))| (*x, i))
        .collect::<Vec<(i32, usize)>>();
    x.sort_unstable();
    let mut x_index = vec![0; n];
    for i in 0..n {
        x_index[x[i].1] = i;
    }

    let mut y = xy
        .iter()
        .enumerate()
        .map(|(i, (_, y))| (*y, i))
        .collect::<Vec<(i32, usize)>>();
    y.sort_unstable();
    let mut y_index = vec![0; n];
    for i in 0..n {
        y_index[y[i].1] = i;
    }

    let mut used = vec![false; n];
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, 0)));

    let mut cost = 0;
    while let Some(Reverse((c, i))) = heap.pop() {
        if used[i] {
            continue;
        }

        cost += c;
        used[i] = true;

        if x_index[i] > 0 && !used[x[x_index[i] - 1].1] {
            heap.push(Reverse((
                x[x_index[i]].0 - x[x_index[i] - 1].0,
                x[x_index[i] - 1].1,
            )))
        }
        if x_index[i] < n - 1 && !used[x[x_index[i] + 1].1] {
            heap.push(Reverse((
                x[x_index[i] + 1].0 - x[x_index[i]].0,
                x[x_index[i] + 1].1,
            )))
        }
        if y_index[i] > 0 && !used[y[y_index[i] - 1].1] {
            heap.push(Reverse((
                y[y_index[i]].0 - y[y_index[i] - 1].0,
                y[y_index[i] - 1].1,
            )))
        }
        if y_index[i] < n - 1 && !used[y[y_index[i] + 1].1] {
            heap.push(Reverse((
                y[y_index[i] + 1].0 - y[y_index[i]].0,
                y[y_index[i] + 1].1,
            )))
        }
    }

    println!("{}", cost);
}
