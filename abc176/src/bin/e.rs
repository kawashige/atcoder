use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        _h: usize,
        _w: usize,
        m: usize,
        bombs: [(usize, usize); m],
    }

    let mut bomb_set = HashSet::new();
    let mut row_count = HashMap::new();
    let mut column_count = HashMap::new();

    for (h, w) in bombs {
        *row_count.entry(h).or_insert(0) += 1;
        *column_count.entry(w).or_insert(0) += 1;
        bomb_set.insert((h, w));
    }

    let mut row_count = row_count.into_iter().collect::<Vec<(usize, usize)>>();
    let mut column_count = column_count.into_iter().collect::<Vec<(usize, usize)>>();
    row_count.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    column_count.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    for i in (0..row_count.len()).take_while(|i| row_count[*i].1 == row_count[0].1) {
        for j in (0..column_count.len()).take_while(|j| column_count[*j].1 == column_count[0].1) {
            if !bomb_set.contains(&(row_count[i].0, column_count[j].0)) {
                println!("{}", row_count[0].1 + column_count[0].1);
                return;
            }
        }
    }
    println!("{}", row_count[0].1 + column_count[0].1 - 1);
}
