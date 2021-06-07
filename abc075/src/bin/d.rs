use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xy: [(i32, i32); n]
    }

    let xs = xy
        .iter()
        .map(|(x, _)| *x)
        .collect::<BTreeSet<i32>>()
        .into_iter()
        .collect::<Vec<i32>>();
    let x_map = xs
        .clone()
        .into_iter()
        .zip(0..)
        .collect::<HashMap<i32, usize>>();
    let ys = xy
        .iter()
        .map(|(_, y)| *y)
        .collect::<BTreeSet<i32>>()
        .into_iter()
        .collect::<Vec<i32>>();
    let y_map = ys
        .clone()
        .into_iter()
        .zip(0..)
        .collect::<HashMap<i32, usize>>();

    let mut grid = vec![vec![0; ys.len()]; xs.len()];

    for (x, y) in xy {
        grid[x_map[&x]][y_map[&y]] += 1;
    }

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if x > 0 {
                grid[x][y] += grid[x - 1][y];
            }
            if y > 0 {
                grid[x][y] += grid[x][y - 1];
            }
            if x > 0 && y > 0 {
                grid[x][y] -= grid[x - 1][y - 1];
            }
        }
    }

    let mut min = std::u64::MAX;
    for x1 in 0..grid.len() {
        for y1 in 0..grid[0].len() {
            for x2 in x1..grid.len() {
                for y2 in y1..grid[0].len() {
                    let mut count = grid[x2][y2];
                    if x1 > 0 && y1 > 0 {
                        count += grid[x1 - 1][y1 - 1];
                    }
                    if x1 > 0 {
                        count -= grid[x1 - 1][y2];
                    }
                    if y1 > 0 {
                        count -= grid[x2][y1 - 1];
                    }

                    if count >= k {
                        min =
                            std::cmp::min(min, (xs[x2] - xs[x1]) as u64 * (ys[y2] - ys[y1]) as u64);
                    }
                }
            }
        }
    }

    println!("{:?}", min);
}
