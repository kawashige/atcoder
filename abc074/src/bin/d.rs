use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n]
    }
    let mut sum: u64 = 0;
    for i in 0..n {
        for j in 0..n {
            sum += a[i][j] as u64;
        }
    }

    let mut set = HashSet::new();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if a[i][j] > a[i][k] + a[k][j] {
                    println!("-1");
                    return;
                } else if a[i][j] == a[i][k] + a[k][j] && i != k && k != j {
                    set.insert((i, j));
                }
            }
        }
    }

    for (i, j) in set {
        sum -= a[i][j] as u64;
    }

    println!("{}", sum / 2);
}
