use std::collections::HashSet;

use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n]
    }

    let mut set = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let d = gcd(
                (xy[i].0 - xy[j].0).abs() as usize,
                (xy[i].1 - xy[j].1).abs() as usize,
            );

            set.insert((
                (xy[i].0 - xy[j].0) / d as i32,
                (xy[i].1 - xy[j].1) / d as i32,
            ));
        }
    }

    println!("{}", set.len());
}
