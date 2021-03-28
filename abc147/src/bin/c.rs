use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
    }

    let mut testimonies = vec![vec![]; n];

    for i in 0..n {
        input! {
            m: usize,
            a: [(usize, usize); m]
        }

        for (x, y) in a {
            testimonies[x - 1].push((i, y));
        }
    }

    let mut min = std::u32::MAX;
    for i in 0..2_usize.pow(n as u32) {
        if testimonies.iter().enumerate().all(|(j, v)| {
            let set = v
                .iter()
                .filter(|(j, _)| i & 1_usize << j == 0)
                .map(|(_, y)| *y)
                .collect::<HashSet<usize>>();

            set.len() == 0
                || (set.len() == 1
                    && *set.iter().next().unwrap() == if i & 1_usize << j == 0 { 1 } else { 0 })
        }) {
            min = std::cmp::min(min, i.count_ones());
        }
    }

    println!("{}", n - min as usize);
}
