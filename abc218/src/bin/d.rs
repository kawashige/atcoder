use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n]
    }

    xy.sort_unstable();

    let set = xy.clone().into_iter().collect::<HashSet<_>>();
    let mut r = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            if xy[i].0 == xy[j].0 || xy[i].1 == xy[j].1 {
                continue;
            }
            if set.contains(&(xy[i].0, xy[j].1)) && set.contains(&(xy[j].0, xy[i].1)) {
                r += 1;
            }
        }
    }

    println!("{}", r / 2);
}
