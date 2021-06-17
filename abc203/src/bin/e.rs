use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut xy: [(usize, usize); m]
    }

    xy.sort_unstable();

    let mut set = HashSet::new();
    set.insert(n);

    let mut i = 0;
    while i < m {
        let mut y = vec![xy[i].1];
        while i < m - 1 && xy[i + 1].0 == xy[i].0 {
            i += 1;
            y.push(xy[i].1)
        }

        let mut remove = vec![];
        let mut insert = vec![];

        for target in y {
            if set.contains(&target) {
                remove.push(target);
            }
            if (target > 0 && set.contains(&(target - 1))) || set.contains(&(target + 1)) {
                insert.push(target);
            }
        }

        for y in remove {
            set.remove(&y);
        }
        for y in insert {
            set.insert(y);
        }
        i += 1;
    }

    println!("{}", set.len());
}
