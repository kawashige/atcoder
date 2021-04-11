use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        d: usize,
        x: [[i32; d]; n]
    }

    let squares = (1..=(16000_f64.sqrt() as i32))
        .map(|i| i * i)
        .collect::<HashSet<i32>>();

    let mut count = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let sum = (0..d).map(|k| (x[i][k] - x[j][k]).pow(2)).sum::<i32>();
            if squares.contains(&sum) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
