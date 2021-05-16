use proconio::input;
use std::collections::HashMap;

fn diff(colors: &HashMap<usize, usize>, c: usize, d: &Vec<Vec<usize>>) -> usize {
    colors
        .iter()
        .map(|(color, count)| d[*color - 1][c - 1] * count)
        .sum()
}

fn main() {
    input! {
        n: usize,
        c: usize,
        d: [[usize; c]; c],
        cc: [[usize; n]; n]
    }

    let mut colors = vec![HashMap::new(); 3];
    for i in 0..n {
        for j in 0..n {
            *colors[(i + j) % 3].entry(cc[i][j]).or_insert(0) += 1;
        }
    }

    let mut min = std::usize::MAX;
    for i in 1..=c {
        let d0 = diff(&colors[0], i, &d);
        for j in 1..=c {
            if i == j {
                continue;
            }
            let d1 = d0 + diff(&colors[1], j, &d);
            for k in 1..=c {
                if i == k || j == k {
                    continue;
                }
                let d2 = d1 + diff(&colors[2], k, &d);
                min = std::cmp::min(min, d2);
            }
        }
    }

    println!("{}", min);
}
